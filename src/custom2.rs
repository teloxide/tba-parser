use crate::{MethodArgs, Parsed, Type};
use chrono::Datelike;
use serde::Serialize;

pub fn generate(parsed: Parsed) -> Schema {
    let methods = parsed.methods.into_iter().map(Method::from).collect();
    let objects = parsed.objects.into_iter().map(Object::from).collect();

    Schema {
        version: Version {
            major: parsed.version.major,
            minor: parsed.version.minor,
            patch: parsed.version.patch,
        },
        recent_changes: Date {
            year: parsed.recent_changes.year(),
            month: parsed.recent_changes.month(),
            day: parsed.recent_changes.day(),
        },
        methods,
        objects,
    }
}

#[derive(Serialize)]
pub struct Schema {
    version: Version,
    recent_changes: Date,
    methods: Vec<Method>,
    objects: Vec<Object>,
}

#[derive(Serialize)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

#[derive(Serialize)]
struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Kind {
    Integer {
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<i64>,
        enumeration: Vec<i64>,
    },
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_len: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_len: Option<u64>,
        enumeration: Vec<String>,
    },
    Bool {
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<bool>,
    },
    Float,
    AnyOf {
        any_of: Vec<KindWrapper>,
    },
    Reference {
        reference: String,
    },
    Array {
        array: Box<KindWrapper>,
    },
}

// this type used to avoid recursion type
// because serde and schemars don't support such types
#[derive(Debug, Serialize)]
#[serde(transparent)]
struct KindWrapper(Kind);

impl From<crate::Type> for KindWrapper {
    fn from(ty: crate::Type) -> Self {
        let base = match ty {
            Type::Integer {
                default,
                min,
                max,
                one_of,
            } => Kind::Integer {
                default,
                min,
                max,
                enumeration: one_of,
            },
            Type::String {
                default,
                min_len,
                max_len,
                one_of,
            } => Kind::String {
                default,
                min_len,
                max_len,
                enumeration: one_of,
            },
            Type::Bool { default } => Kind::Bool { default },
            Type::Float => Kind::Float,
            Type::Or(types) => Kind::AnyOf {
                any_of: types.into_iter().map(KindWrapper::from).collect(),
            },
            Type::Object(object) => Kind::Reference { reference: object },
            Type::Array(ty) => Kind::Array {
                array: Box::new(KindWrapper::from(*ty)),
            },
        };
        KindWrapper(base)
    }
}

#[derive(Serialize)]
struct Method {
    name: String,
    description: String,
    arguments: Vec<Argument>,
    maybe_multipart: bool,
    return_type: KindWrapper,
    documentation_link: String,
}

impl From<crate::Method> for Method {
    fn from(method: crate::Method) -> Self {
        let (maybe_multipart, args) = match method.args {
            MethodArgs::No => (false, vec![]),
            MethodArgs::Yes(args) => (false, args),
            MethodArgs::WithMultipart(args) => (true, args),
        };
        Self {
            name: method.name,
            description: method.description,
            arguments: args.into_iter().map(Argument::from).collect(),
            maybe_multipart,
            return_type: KindWrapper::from(method.return_type),
            documentation_link: method.docs_link,
        }
    }
}

#[derive(Serialize)]
struct Argument {
    name: String,
    description: String,
    required: bool,
    #[serde(rename = "type_info")]
    kind: KindWrapper,
}

impl From<crate::Argument> for Argument {
    fn from(arg: crate::Argument) -> Self {
        Self {
            name: arg.name,
            description: arg.description,
            required: arg.required,
            kind: KindWrapper::from(arg.kind),
        }
    }
}

#[derive(Serialize)]
struct Object {
    name: String,
    description: String,
    #[serde(flatten)]
    data: ObjectData,
    documentation_link: String,
}

impl From<crate::Object> for Object {
    fn from(object: crate::Object) -> Self {
        Self {
            name: object.name,
            description: object.description,
            data: ObjectData::from(object.data),
            documentation_link: object.docs_link,
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum ObjectData {
    Properties { properties: Vec<Property> },
    AnyOf { any_of: Vec<KindWrapper> },
    Unknown,
}

impl From<crate::ObjectData> for ObjectData {
    fn from(object_data: crate::ObjectData) -> Self {
        match object_data {
            crate::ObjectData::Fields(fields) => ObjectData::Properties {
                properties: fields.into_iter().map(Property::from).collect(),
            },
            crate::ObjectData::Elements(types) => ObjectData::AnyOf {
                any_of: types.into_iter().map(KindWrapper::from).collect(),
            },
            crate::ObjectData::Unknown => ObjectData::Unknown,
        }
    }
}

#[derive(Serialize)]
struct Property {
    name: String,
    description: String,
    required: bool,
    #[serde(rename = "type_info")]
    kind: KindWrapper,
}

impl From<crate::Field> for Property {
    fn from(field: crate::Field) -> Self {
        Self {
            name: field.name,
            description: field.description,
            required: field.required,
            kind: KindWrapper::from(field.kind),
        }
    }
}

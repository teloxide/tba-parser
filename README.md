# tba-parser

This is a fork of [ENCRYPTEDFOREVER](https://github.com/ENCRYPTEDFOREVER/tg-bot-api) that is a fork of an amazing project [tg-bot-api](https://github.com/ark0f/tg-bot-api)

The original project is unmaintained, so the we decided to fork it, fix some bugs and prune all of the useless for us stuff. The main goal of this fork is to allow schema checking and rust types checking, but custom_v2.json schema should be true to the original (all other schemas were removed, but if you want you can add them back yourself pretty easily).

## Usage

Just clone the repo and run `cargo run` for the latest schema. The schema will be at `schema/custom_v2.json` If you want schema of an earlier version of the TBA, use web archive link as an argument: `cargo run -- https://web.archive.org/web/20250817052931/https://core.telegram.org/bots/api`

Documentation on the schema can be found at [CUSTOM_SCHEMA.md](CUSTOM_SCHEMA.md).

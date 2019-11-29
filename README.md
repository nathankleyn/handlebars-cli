# Handlebars CLI [![Build Status](https://travis-ci.org/nathankleyn/handlebars-cli.svg)](https://travis-ci.org/nathankleyn/handlebars-cli) [![Crates.io Version Of handlebars-cli](https://img.shields.io/crates/v/handlebars-cli.svg)](https://crates.io/crates/handlebars-cli)

Template JSON properties into Handlebars templates from the CLI.

## Install

You can easily install this using Cargo:

```
cargo install handlebars-cli
```

You can also do a simple `cargo build --release` in the cloned out version of this repository to get a binary (only stable Rust is required).

## Usage

```
$ handlebars-cli --help
handlebars-cli — Template JSON properties into Handlebars templates from the CLI.

USAGE:
    handlebars-cli <JSON> <TEMPLATE>
    handlebars-cli --help

PARAMETERS:
    JSON: A set of valid JSON to use as properties to interpolate into the provided template file.
    TEMPLATE: A path to a valid Handlebars template.

FLAGS:
    --help: Prints this usage text.
```

## Example

```bash
$ echo "Hello {{name.first}} {{name.last}}!" > template.hbs
$ handlebars-cli '{ "name": { "first": "Foo", "last": "Bar" }}' template.hbs
Hello Foo Bar!
```

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

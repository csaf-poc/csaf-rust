# csaf-rust

This repository is a proof-of-concept for a CSAF library in Rust that relies on automatically generating CSAF document structs from the JSON schema.

This is work-in-progress.

## Build

If you want to build `csaf-validator` on your own, please install Rust (see https://rustup.rs) and then run

```bash
# make sure submodules are up-to-date
git submodule init
git submodule update --remote

# make sure that local assets are in sync with git submodules
./update_assets.sh

# run the tests
cargo test

# build for release
cargo build --release
```

The final binary will be in `target/release` and can then be installed, for example, in a system-wide folder.

## Usage

After [building](README.md#build) or downloading `csaf-validator` from [the available releases](https://github.com/csaf-rs/csaf/releases), the usage is quite simple and additional help can be display using `--help`.

```
A validator for CSAF documents

Usage: csaf-validator [OPTIONS] <PATH>

Arguments:
  <PATH>  

Options:
  -c, --csaf-version <CSAF_VERSION>  Version of CSAF to use [default: 2.0]
  -p, --preset <PRESET>              The validation preset to use [default: basic]
  -t, --test-id <TEST_ID>            Run only the selected tests, may be specified multiple times
  -h, --help                         Print help
  -V, --version                      Print version
```

Some examples to use are included below. Please note that the validation is not yet fully implemented!

```bash
# validate a CSAF 2.0 document with profile basic (the default)
csaf-validator --csaf-version 2.0 my-csaf-2-0-document.json

# validate a CSAF 2.0 document with profile full
csaf-validator --csaf-version 2.0 --preset full my-csaf-2-0-document.json

# validate a CSAF 2.1 document with one specific test
csaf-validator --csaf-version 2.1 --test-id 6.1.34 my-csaf-2-1-document.json
```

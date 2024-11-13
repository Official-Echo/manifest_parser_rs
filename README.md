# manifest_parser_rs

## General Description
The `manifest_parser_rs` is a Rust library designed to parse Cargo.toml manifest files.

## Parsing 

The grammar for `manifest_parser_rs` is defined using Pest, capturing various components essential for accurately interpreting Cargo manifest files. This includes special handling of dependencies and version specifications, with rigorous validation to ensure versions follow SemVer compliance. The main components of the grammar are as follows:

- **Sections**: Sections are identified by square brackets and can include common Cargo sections such as `[dependencies]`, `[dev-dependencies]`, `[package]`, and more specialized sections like `[features]` and `[profile]`.
- **Key-Value Pairs**: Standard key-value pairs are in the format `key = value`, with support for various value types (e.g., strings, numbers, and special characters).
- **Comments**: Comments begin with `#` and are ignored by the parser.
- **Dependencies Parsing**: The parser handles dependencies within both `[dependencies]` and `[dev-dependencies]` sections. Each dependency entry may specify:
  - `version`: With enforced SemVer compliance, versions are checked to ensure they adhere to Semantic Versioning 2.0 (e.g., `1.2.3`, `1.2.3-beta.1`).
  - `git`: A git repository URL as a source.
  - `path`: A file path to the local dependency.
  - `registry`: A specified registry for the dependency.
  - `workspace` and `optional` flags, along with specific `features`.
- **Version Compliance**: Versions in dependency declarations are validated to confirm alignment with SemVer, including support for pre-release identifiers (e.g., `-beta`, `-rc.1`) and build metadata (e.g., `+build.5`).

### The grammar components defined in the Pest grammar include:

- **Whitespace and Comments**: Ignoring spaces, tabs, line breaks, and comments.
- **Section Names**: Recognized section headers include standard sections (`[dependencies]`, `[package]`, etc.) and custom sections specified in the grammar.
- **Identifiers and Key-Value Definitions**: Allowing alphanumeric identifiers with additional special characters.
- **Version Format**: Structured to allow semantic versioning with optional pre-release and build metadata.
  
### The parsing process involves:
1. **Tokenization**: Initial parsing of the input file into tokens according to Pest grammar rules.
2. **Section Identification**: Distinguishing and processing each section.
3. **Extraction of Key-Value Pairs**: Parsing the entries within each section.
4. **Storing Results**: Data is stored in a hashmap, making it accessible for further use.

This structured approach ensures that the parser can handle complex manifest structures while maintaining strict versioning and dependency requirements as outlined by Cargo and SemVer.

## Example

```toml
[package]
name = "manifest_parser_rs"
version = "0.2.0"
edition = "2021"
description = "A Cargo.toml manifest file parser that supports sections and key-value pairs"
authors = ["Official-Echo <official-echo@github.com>"]
license = "MIT"
repository = "https://github.com/Official-Echo/manifest_parser_rs"
#documentation = "https://docs.rs/manifest_parser_rs"
readme = "README.md"
keywords = ["manifest", "parser", "ini", "configuration", "settings"]
categories = ["parsing", "manifest"]

[dependencies]
anyhow = "1.0.93"
clap = { version = "4.5.20", features = ["derive"] }
pest = "2.7.14"
pest_derive = "2.7.14"
thiserror = "2.0.3"

[lib]
name = "manifest_parser_rs"
path = "src/lib.rs"

[[bin]]
name = "manifest_parser_rs"
path = "src/main.rs"

```
```text
	|||  
	|||  
	|||  
	|||  
	|||  
	|||  
	|||  
  \	||| /
   \\|//  
	\|/  
	 v  
Hashmap of 
 ├─ package
 │  ├─ name "manifest_parser_rs"
 │  ├─ version "0.2.0"
 │  ├─ edition "2021"
 │  ├─ description "A Cargo.toml manifest file parser..."
 │  ├─ authors ["Official-Echo <official-echo@github.com>"]
 │  ├─ license "MIT"
 │  ├─ repository "https://github.com/Official-Echo/manifest_parser_rs"
 │  ├─ readme "README.md"
 │  ├─ keywords ["manifest", "parser", "ini", "configuration", "settings"]
 │  └─ categories ["parsing", "manifest"]
 │
 ├─ dependencies
 │  ├─ anyhow "1.0.93"
 │  ├─ clap { version = "4.5.20", features = ["derive"] }
 │  ├─ pest "2.7.14"
 │  ├─ pest_derive "2.7.14"
 │  └─ thiserror "2.0.3"
 │
 ├─ lib
 │  ├─ name "manifest_parser_rs"
 │  └─ path "src/lib.rs"
 │
 └─ bin
    ├─ name "manifest_parser_rs"
    └─ path "src/main.rs"
 ```
# Manifest Parser Grammar

This documentation explains the rules and structure of the manifest parser grammar used in this project.

## Core Rules

### WHITESPACE and COMMENTS

```pest
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
COMMENT    = _{ "#" ~ (!NEWLINE ~ ANY)* ~ (NEWLINE | EOI) }
```

These rules handle basic formatting:

- `WHITESPACE`: Matches any whitespace characters (space, tab, return, newline)
- `COMMENT`: Matches lines starting with `#` until the end of line

### Version Rules

```pest
version_core = _{ numeric_identifier ~ "." ~ numeric_identifier ~ "." ~ numeric_identifier }
version =  { "\"" ~ version_core ~ ("-" ~ pre_release)? ~ ("+" ~ build)? ~ "\"" }
```

Handles semantic versioning (SemVer) format:

- `version_core`: Matches `MAJOR.MINOR.PATCH` format
- `version`: Full version string including pre-release and build metadata
- Example: `"1.2.3"`, `"1.2.3-alpha"`, `"1.2.3+build.123"`

### Section Names

```pest
section_name = @{
    "lib"
  | "bin"
  | "example"
  | "test"
  | "bench"
  | "build-dependencies"
  | "target"
  | "badges"
  | "features"
  | "lints"
  | "patch"
  | "replace"
  | "profile"
  | "workspace"
}
```

Predefined section names allowed in the manifest.

### Identifier Rules

```pest
non_digit = _{ ASCII_ALPHA | "-" }
identifier_characters = _{ (ASCII_DIGIT | non_digit)+ }
```

Rules for parsing identifiers:

- `non_digit`: Any alphabetic character or hyphen
- `identifier_characters`: Combination of digits and non-digits

## Section Structure

### Package Section

```pest
package_section = {
    "[package]" ~ "name" ~ "=" ~ value ~ "version" ~ "=" ~ version ~ section_inside
}
```

The required package section containing:

- Package name
- Version number
- Additional metadata

### Dependencies Section

```pest
dependencies_section = {
    ("[dependencies]" | "[dev-dependencies]") ~ dependencies_key_value*
}
```

Dependencies section with various dependency specifications:

```pest
dependency_spec = {
    "{" ~ (
        dependency_version |
        dependency_git |
        dependency_path |
        dependency_registry |
        dependency_workspace |
        dependency_optional |
        features
    )+ ~ "}"
}
```

Example dependency formats:

```toml
[dependencies]
simple = "1.0.0"
complex = { version = "1.0.0", features = ["async"] }
local = { path = "../local-dep" }
git-dep = { git = "https://github.com/user/repo" }
```

### Key-Value Pairs

```pest
key = @{ (ASCII_ALPHANUMERIC | "_" | "-")+ }
value = @{ (!NEWLINE ~ WHITESPACE* ~ possible_value_char)+ }
key_value = { key ~ "=" ~ value }
```

Basic key-value pair structure:

- `key`: Alphanumeric characters with underscores and hyphens
- `value`: Any characters except newline
- Example: `key = "value"`

### Section Structure

```pest
section_definition = { "[" ~ "["? ~ section_name ~ "]" ~ "]"? }
section_inside = { key_value* }
section = { section_definition ~ section_inside }
```

General section structure supporting both single and double brackets:

- Single bracket: `[section]`
- Double bracket: `[[section]]`

### Main Manifest Rule

```pest
manifest = {
    SOI ~ package_section ~ (section | dependencies_section)* ~ EOI
}
```

Complete manifest structure:

1. Start of input
2. Required package section
3. Optional additional sections
4. End of input

## Examples

### Basic Manifest

```toml
[package]
name = "my-package"
version = "1.0.0"
edition = "2021"

[dependencies]
serde = "1.0.0"
tokio = { version = "1.0.0", features = ["full"] }
```

### Complex Manifest

```toml
[package]
name = "complex-package"
version = "0.1.0-alpha+build.123"
edition = "2021"

[dependencies]
local-dep = { path = "../local" }
git-dep = { git = "https://github.com/user/repo" }

[[bin]]
name = "tool"
path = "src/main.rs"

[features]
default = ["async"]
async = ["tokio"]
```

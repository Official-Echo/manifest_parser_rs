> **Warning:** Work In Progress

# manifest_parser_rs

### General Description
The `manifest_parser_rs` is a work-in-progress Rust library designed to parse Cargo.toml manifest files (now supports only a simple key-value format with support for sections). It recognizes sections marked with square brackets (e.g., `[section_name]`), key-value pairs in the format `key = value`, and comments starting with `#`.

### Technical Description
This parser processes configuration files in a simple key-value format with section support. The parser recognizes:
- Sections marked with square brackets: `[section_name]`
- Key-value pairs in format: `key` `=` `value`
- Comments starting with `#`

### Parsing Process
1. The parser first tokenizes the input file using Pest grammar rules
2. Each section is identified and processed	
3. Within each section, key-value pairs are extracted
4. The results are stored in pair for further use
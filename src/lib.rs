use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct ManifestParser;

#[derive(Debug)]
pub enum ManifestError {
    ParseError(String),
    MissingSection(String),
    MissingKey(String, String),
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ManifestError::MissingSection(section) => write!(f, "Missing section: {}", section),
            ManifestError::MissingKey(section, key) => {
                write!(f, "Missing key {} in section {}", key, section)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Manifest {
    sections: HashMap<String, HashMap<String, String>>,
}
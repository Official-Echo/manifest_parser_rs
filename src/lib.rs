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

impl Manifest {
    pub fn parse(input: &str) -> Result<Self, ManifestError> {
        let mut manifest = Manifest::default();
        let parsed_item = ManifestParser::parse(Rule::manifest, input)
            .map_err(|e| ManifestError::ParseError(e.to_string()))?;

        let mut current_item = None;

        for item in parsed_item.flatten() {
            match item.as_rule() {
                Rule::section => {
                    let inner = item.into_inner().next().unwrap(); //next section
                    current_item = Some(inner.as_str().trim().to_string());
                    manifest
                        .sections
                        .insert(current_item.clone().unwrap(), HashMap::new());
                }
                Rule::key_value => {
                    let mut inner = item.into_inner();
                    let key = inner.next().unwrap().as_str().trim();
                    let value = inner.next().unwrap().as_str().trim();

                    if let Some(section) = &current_item {
                        if let Some(section_map) = manifest.sections.get_mut(section) {
                            section_map.insert(key.to_string(), value.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(manifest)
    }

    pub fn get(&self, section: &str, key: &str) -> Result<&str, ManifestError> {
        let picked_section = self
            .sections
            .get(section)
            .ok_or_else(|| ManifestError::MissingSection(section.to_string()))?;

        let retrieved_value = picked_section
            .get(key)
            .ok_or_else(|| ManifestError::MissingKey(section.to_string(), key.to_string()))?;

        Ok(retrieved_value)
    }
}

#![doc = include_str!("../docs.md")]
//! A parser for manifest files using Pest grammar.

use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

/// The main parser for manifest files.
/// This parser reads and validates manifest files that define sections
/// including package information and dependencies.
///
/// # Example
///
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ManifestParser;

/// Represents errors that can occur during manifest parsing.
#[derive(Debug, Error)]
pub enum ManifestError {
    /// Represents a parsing error with details
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Indicates a missing section in the manifest
    #[error("Missing section: {0}")]
    MissingSection(String),

    /// Indicates a missing key within a section
    #[error("Missing key {1} in section {0}")]
    MissingKey(String, String),
}

/// Represents a parsed manifest containing sections of key-value pairs.
#[derive(Debug, Default)]
pub struct Manifest {
    /// Map of section names to their key-value pairs
    sections: HashMap<String, HashMap<String, String>>,
}

impl Manifest {
    /// Parses a manifest string into a structured format.
    ///
    /// # Arguments
    ///
    /// * `input` - The manifest content as a string
    ///
    /// # Returns
    ///
    /// A `Result` containing either a parsed `Manifest` or a `ManifestError`
    pub fn parse(input: &str) -> Result<Self, ManifestError> {
        let mut manifest = Manifest::default();
        let parsed_item = ManifestParser::parse(Rule::manifest, input)
            .map_err(|e| ManifestError::ParseError(e.to_string()))?;
        let mut current_section = None;

        for item in parsed_item.flatten() {
            match item.as_rule() {
                Rule::section => {
                    current_section = parse_section(item);
                    manifest
                        .sections
                        .insert(current_section.clone().unwrap(), HashMap::new());
                }
                Rule::key_value => {
                    parse_key_value(item, &mut manifest, &current_section)?;
                }
                Rule::package_section => {
                    parse_package_section(item, &mut manifest)?;
                }
                Rule::dependencies_section => {
                    parse_dependencies_section(item, &mut manifest)?;
                }
                _ => {}
            }
        }

        Ok(manifest)
    }

    /// Retrieves an iterator over the section names in the manifest.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to the section names as strings.
    pub fn sections(&self) -> impl Iterator<Item = &String> {
        self.sections.keys()
    }

    /// Retrieves a value from the manifest given a section and key.
    ///
    /// # Arguments
    ///
    /// * `section` - The section name
    /// * `key` - The key within the section
    ///
    /// # Returns
    ///
    /// A `Result` containing either the value as a string slice or a `ManifestError`
    pub fn get_by_key(&self, section: &str, key: &str) -> Result<&str, ManifestError> {
        let section_map = self
            .sections
            .get(section)
            .ok_or_else(|| ManifestError::MissingSection(section.to_string()))?;

        let value = section_map
            .get(key)
            .ok_or_else(|| ManifestError::MissingKey(section.to_string(), key.to_string()))?;

        Ok(value)
    }

    /// Retrieves a section from the manifest as a map of key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `section` - The section name
    ///
    /// # Returns
    ///
    /// A `Result` containing either the section map or a `ManifestError`
    pub fn get_by_section(&self, section: &str) -> Result<&HashMap<String, String>, ManifestError> {
        let section_map = self
            .sections
            .get(section)
            .ok_or_else(|| ManifestError::MissingSection(section.to_string()))?;

        Ok(section_map)
    }
}

/// Parses a section name from the manifest.
fn parse_section(item: pest::iterators::Pair<Rule>) -> Option<String> {
    let inner = item.into_inner().next().unwrap();
    let section_name = inner.as_str().trim().to_string();

    if section_name.starts_with('[') && section_name.ends_with(']') {
        Some(
            section_name
                .trim_matches(|c| c == '[' || c == ']')
                .to_string(),
        )
    } else {
        Some(section_name)
    }
}

/// Parses a key-value pair from the manifest.
fn parse_key_value(
    item: pest::iterators::Pair<Rule>,
    manifest: &mut Manifest,
    current_section: &Option<String>,
) -> Result<(), ManifestError> {
    let mut inner = item.into_inner();
    let key = inner.next().unwrap().as_str().trim();
    let value = inner.next().unwrap().as_str().trim().trim_matches('"');

    if let Some(section) = current_section {
        if let Some(section_map) = manifest.sections.get_mut(section) {
            section_map.insert(key.to_string(), value.to_string());
        }
    }

    Ok(())
}

/// Parses a package section from the manifest.
fn parse_package_section(
    item: pest::iterators::Pair<Rule>,
    manifest: &mut Manifest,
) -> Result<(), ManifestError> {
    let section_name = "package".to_string();
    manifest
        .sections
        .insert(section_name.clone(), HashMap::new());

    let mut inner = item.into_inner();
    let name = inner.next().unwrap().as_str().trim();
    let version = inner.next().unwrap().as_str().trim();

    let section_map = manifest.sections.get_mut(&section_name).unwrap();
    section_map.insert("name".to_string(), name.trim_matches('"').to_string());
    section_map.insert("version".to_string(), version.trim_matches('"').to_string());

    //Check if there are items left
    let inner = inner.next().unwrap().into_inner();

    for item in inner {
        let mut inner = item.into_inner();
        let key = inner.next().unwrap().as_str().trim();
        let value = inner.next().unwrap().as_str().trim().trim_matches('"');
        section_map.insert(key.to_string(), value.to_string());
    }

    Ok(())
}

/// Parses a dependencies section from the manifest.
fn parse_dependencies_section(
    item: pest::iterators::Pair<Rule>,
    manifest: &mut Manifest,
) -> Result<(), ManifestError> {
    let mut section_name = "dependencies".to_string();

    if manifest.sections.contains_key(&section_name) {
        section_name = "dev-dependencies".to_string();
    }
    manifest
        .sections
        .insert(section_name.clone(), HashMap::new());

    for dep in item.into_inner() {
        let mut inner = dep.into_inner();
        let key = inner.next().unwrap().as_str().trim();
        let value = inner.next().unwrap().as_str().trim().trim_matches('"');

        let section_map = manifest.sections.get_mut(&section_name).unwrap();
        section_map.insert(key.to_string(), value.to_string());
    }

    Ok(())
}

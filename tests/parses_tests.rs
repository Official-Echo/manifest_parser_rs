use anyhow::Result;
use manifest_parser_rs::*;
use pest::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    mod get_by_key {
        use super::*;

        #[test]
        fn valid_key_queries() -> Result<()> {
            let manifest = Manifest::parse(
                r#"
                [package]
                name = "test"
                version = "1.0.0"
                
                [dependencies]
                serde = "1.0.0"
            "#,
            )?;

            assert_eq!(manifest.get_by_key("package", "name")?, "test");
            assert_eq!(manifest.get_by_key("package", "version")?, "1.0.0");
            assert_eq!(manifest.get_by_key("dependencies", "serde")?, "1.0.0");

            Ok(())
        }

        #[test]
        fn invalid_key_queries() {
            let manifest = Manifest::parse(
                r#"
                [package]
                name = "test"
                version = "1.0.0"
            "#,
            )
            .unwrap();

            assert!(matches!(
                manifest.get_by_key("invalid", "key"),
                Err(ManifestError::MissingSection(_))
            ));

            assert!(matches!(
                manifest.get_by_key("package", "invalid"),
                Err(ManifestError::MissingKey(_, _))
            ));
        }
    }

    mod get_by_section {
        use super::*;

        #[test]
        fn valid_section_queries() -> Result<()> {
            let manifest = Manifest::parse(
                r#"
                [package]
                name = "test"
                version = "1.0.0"
            "#,
            )?;

            let section = manifest.get_by_section("package")?;
            assert_eq!(section.get("name"), Some(&"test".to_string()));
            assert_eq!(section.get("version"), Some(&"1.0.0".to_string()));

            Ok(())
        }

        #[test]
        fn invalid_section_queries() {
            let manifest = Manifest::parse(
                r#"
                [package]
                name = "test"
				version = "1.0.0"
            "#,
            )
            .unwrap();

            assert!(matches!(
                manifest.get_by_section("invalid"),
                Err(ManifestError::MissingSection(_))
            ));
        }
    }

    mod parsing {
        use super::*;

        #[test]
        fn valid_package_section() -> Result<()> {
            let input = r#"[package]
            name = "test"
            version = "1.0.0"
            description = "A test package""#;

            let pair = ManifestParser::parse(Rule::package_section, input)?
                .next()
                .unwrap();
            assert!(pair.as_str().contains("name"));
            assert!(pair.as_str().contains("version"));

            Ok(())
        }

        #[test]
        fn valid_dependencies_section() -> Result<()> {
            let input = r#"[dependencies]
            serde = "1.0.0"
            tokio = { version = "1.0.0" }"#;

            let pair = ManifestParser::parse(Rule::dependencies_section, input)?
                .next()
                .unwrap();

            assert!(pair.as_str().contains("tokio"));
            assert!(pair.as_str().contains("tokio"));

            Ok(())
        }

        #[test]
        fn valid_section() -> Result<()> {
            let inputs = [
                r#"[lib]
                name = "mylib"
                path = "src/lib.rs""#,
                r#"[[bin]]
                name = "mycli"
                path = "src/main.rs""#,
            ];

            for input in inputs {
                let pair = ManifestParser::parse(Rule::section, input)?.next().unwrap();
                assert!(pair.as_str().contains("name"));
            }

            Ok(())
        }

        #[test]
        fn invalid_sections() {
            let invalid_inputs = [
                // Missing required fields
                r#"[package]
                name = "test""#,
                // Invalid section name
                r#"[invalid]
                key = "value""#,
                // Malformed section
                r#"[dependencies
                serde = "1.0""#,
            ];

            for input in invalid_inputs {
                assert!(
                    ManifestParser::parse(Rule::manifest, input).is_err(),
                    "Should fail: {}",
                    input
                );
            }
        }

        #[test]
        fn verbose_dependencies_section() -> Result<()> {
            let input = r#"[dependencies]
			serde = "1.0.0"
			tokio = { version = "1.0.0", features = ["full"], optional = true }"#;

            let pair = ManifestParser::parse(Rule::dependencies_section, input)?
                .next()
                .unwrap();

            assert!(pair.as_str().contains("tokio"));
            assert!(pair.as_str().contains("full"));
            assert!(pair.as_str().contains("optional"));

            Ok(())
        }

        #[test]
        fn version_correct_examples() -> Result<()> {
            let inputs = [
                "\"1.0.0\"",
                "\"1.0.0-alpha\"",
                "\"1.0.0-alpha.1\"",
                "\"1.0.0-alpha+001\"",
                "\"1.0.0+20130313144700\"",
                "\"1.0.0-beta+exp.sha.5114f85\"",
                "\"1.0.0+21AF26D3--117B344092BD\"",
            ];

            for input in inputs {
                let pair = ManifestParser::parse(Rule::version, input)?.next().unwrap();
                assert_eq!(pair.as_str(), input);
            }

            Ok(())
        }

        #[test]
        fn incorrect_version_examples() -> Result<()> {
            let inputs = [
                "\"1.0\"",
                "\"1.0.0+\"",
                "\"1.0.0+20130313144700+\"",
                "\"1.0.0+21AF26D3--117B344092BD+\"",
                "\"1.1.1.1.1.1.1\"",
                "\"word\"",
            ];

            for input in inputs {
                assert!(ManifestParser::parse(Rule::version, input).is_err());
            }

            Ok(())
        }
    }

    mod extended_parsing {
        use super::*;

        #[test]
        fn features_with_multiple_entries() -> Result<()> {
            let input = r#"[dependencies]
			tokio = { version = "1.0.0", features = ["full", "io-std", "net"] }
			"#;

            let pair = ManifestParser::parse(Rule::dependencies_section, input)?
                .next()
                .unwrap();

            assert!(pair.as_str().contains("features"));
            assert!(pair.as_str().contains("full"));
            assert!(pair.as_str().contains("io-std"));
            assert!(pair.as_str().contains("net"));

            Ok(())
        }

        #[test]
        fn dependency_spec_edge_cases() -> Result<()> {
            let inputs = [
                r#"[dependencies]
				crate_a = { version = "1.0.0", git = "https://mohylyanka.com/some.git" }"#,
                r#"[dependencies]
				crate_b = { path = "../local/path" }"#,
                r#"[dependencies]
				crate_c = { registry = "custom", workspace = true, optional = true }"#,
            ];

            for input in inputs {
                let pair = ManifestParser::parse(Rule::dependencies_section, input)?
                    .next()
                    .unwrap();
                assert!(pair.as_str().contains("crate"));
            }

            Ok(())
        }

        #[test]
        fn invalid_numeric_identifier_in_version() {
            let invalid_versions = ["\"001.1.0\"", "\"1.01.0\"", "\"0.0.01\""];

            for version in invalid_versions {
                assert!(
                    ManifestParser::parse(Rule::version, version).is_err(),
                    "Should fail for invalid version: {}",
                    version
                );
            }
        }

        #[test]
        fn all_section_names_coverage() -> Result<()> {
            let section_inputs = [
                r#"[lib]"#,
                r#"[bin]"#,
                r#"[example]"#,
                r#"[test]"#,
                r#"[bench]"#,
                r#"[build-dependencies]"#,
                r#"[target]"#,
                r#"[badges]"#,
                r#"[features]"#,
                r#"[lints]"#,
                r#"[patch]"#,
                r#"[replace]"#,
                r#"[profile]"#,
                r#"[workspace]"#,
            ];

            for input in section_inputs {
                let pair = ManifestParser::parse(Rule::section_definition, input)?
                    .next()
                    .unwrap();
                assert!(pair.as_str().starts_with("["));
            }

            Ok(())
        }

        #[test]
        fn comment_parsing() -> Result<()> {
            let input = r#"
        # Test
        [package]
        name = "test" # Test
        version = "1.0.0"
        "#;

            let manifest = Manifest::parse(input)?;
            assert_eq!(manifest.get_by_key("package", "name")?, "test");
            assert_eq!(manifest.get_by_key("package", "version")?, "1.0.0");
            Ok(())
        }

        #[test]
        fn possible_value_characters() -> Result<()> {
            let input = r#"[package]
        name = "test"
        version = "1.0.0"
        description = "Symbols: []<>@:\\/,-_\""
        "#;

            let pair = ManifestParser::parse(Rule::package_section, input)?
                .next()
                .unwrap();
            assert!(pair.as_str().contains("description"));
            assert!(pair.as_str().contains("Symbols: []<>@:\\\\/,-_\\\""));

            Ok(())
        }

        #[test]
        fn whitespace_flexibility() -> Result<()> {
            let input = r#"[package]
			name    =    "test"
			version = "1.0.0""#;

            let manifest = Manifest::parse(input)?;
            assert_eq!(manifest.get_by_key("package", "name")?, "test");
            assert_eq!(manifest.get_by_key("package", "version")?, "1.0.0");
            Ok(())
        }
    }

    mod inside {
        use super::*;
        #[test]
        fn extended_version_parsing() -> Result<()> {
            let versions = [
                "\"1.2.3\"",
                "\"1.2.3-alpha\"",
                "\"1.2.3+build123\"",
                "\"1.2.3-alpha+build123\"",
            ];

            for version in versions.iter() {
                let input = format!(
                    r#"[package]
       				name = "test"
        			version = {}
        			"#,
                    version
                );

                let manifest = Manifest::parse(&input)?;
                assert_eq!(
                    manifest.get_by_key("package", "version")?,
                    version.trim_matches('"')
                );
            }
            Ok(())
        }

        #[test]
        fn complex_dependency_spec() -> Result<()> {
            let input = r#"[package]
			name = "test"
			version = "1.0.0"
			
			[dependencies]
			mycrate = { version = "1.0.0", git = "https://mohylyanka.com/rust.git", path = "../local/path", registry = "custom", optional = true, workspace = true, features = ["full", "io-std", "net"] }
			"#;

            let manifest = Manifest::parse(input)?;
            let dependency = manifest.get_by_key("dependencies", "mycrate")?;
            assert!(dependency.contains("version"));
            assert!(dependency.contains("git"));
            assert!(dependency.contains("path"));
            assert!(dependency.contains("registry"));
            assert!(dependency.contains("optional"));
            assert!(dependency.contains("workspace"));
            assert!(dependency.contains("features"));
            Ok(())
        }

        #[test]
        fn generic_section_inside() -> Result<()> {
            let input = r#"[package]
			name = "test"
			version = "1.0.0"

    		[profile]
    		opt-level = 2
    		debug = true
    		overflow-checks = true
			supreme-overflows = false
    		"#;

            let manifest = Manifest::parse(input)?;
            let profile_section = manifest.get_by_section("profile")?;
            assert_eq!(profile_section.get("opt-level"), Some(&"2".to_string()));
            assert_eq!(profile_section.get("debug"), Some(&"true".to_string()));
            assert_eq!(
                profile_section.get("overflow-checks"),
                Some(&"true".to_string())
            );
            assert_eq!(
                profile_section.get("supreme-overflows"),
                Some(&"false".to_string())
            );
            Ok(())
        }
    }

    #[test]
    fn full_manifest_parsing() -> Result<()> {
        let input = r#"
        [package]
        name = "full-test"
        version = "1.0.0"
        edition = "2021"

        [dependencies]
        serde = "1.0.0"
        tokio = { version = "1.0.0", features = ["full"] }

        [lib]
        name = "mylib"
        path = "src/lib.rs"

        [[bin]]
        name = "mycli"
        path = "src/main.rs"
        "#;

        let manifest = Manifest::parse(input)?;

        // Test package section
        assert_eq!(manifest.get_by_key("package", "name")?, "full-test");
        assert_eq!(manifest.get_by_key("package", "version")?, "1.0.0");

        // Test dependencies
        assert_eq!(manifest.get_by_key("dependencies", "serde")?, "1.0.0");

        // Test lib section
        assert_eq!(manifest.get_by_key("lib", "name")?, "mylib");

        // Test bin section
        assert_eq!(manifest.get_by_key("bin", "name")?, "mycli");

        Ok(())
    }

    #[test]
    fn sections_iterator() -> Result<()> {
        let manifest = Manifest::parse(
            r#"
            [package]
            name = "test"
            version = "1.0.0"
            
            [dependencies]
            serde = "1.0.0"
            
            [lib]
            name = "mylib"
        "#,
        )?;

        let sections: Vec<&String> = manifest.sections().collect();
        assert!(sections.contains(&&"package".to_string()));
        assert!(sections.contains(&&"dependencies".to_string()));
        assert!(sections.contains(&&"lib".to_string()));

        Ok(())
    }
}

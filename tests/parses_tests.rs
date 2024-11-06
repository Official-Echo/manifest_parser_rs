use manifest_parser_rs::{Manifest, ManifestError};

#[test]
fn test_basic_manifest() {
    let hardcoded_test = r#"
        [mohylyanka]
        ipz = best
        best_discipline = Rust
    "#;

    let manifest = Manifest::parse(hardcoded_test).unwrap();
    assert_eq!(manifest.get("mohylyanka", "ipz").unwrap(), "best");
    assert_eq!(
        manifest.get("mohylyanka", "best_discipline").unwrap(),
        "Rust"
    );
}

#[test]
fn test_multiple_sections() {
    let hardcoded_test = r#"
        [mohylyanka]
        ipz = best

        [why_use_google_if_you_have_bing]
        url = https://bing.com
		best_search_engine = Bing
		greatest_search_engine = Bing
		most_valiant_search_engine = Bing
    "#;

    let manifest = Manifest::parse(hardcoded_test).unwrap();
    assert_eq!(manifest.get("mohylyanka", "ipz").unwrap(), "best");
    assert_eq!(
        manifest
            .get("why_use_google_if_you_have_bing", "url")
            .unwrap(),
        "https://bing.com"
    );
    assert_eq!(
        manifest
            .get("why_use_google_if_you_have_bing", "best_search_engine")
            .unwrap(),
        "Bing"
    );
    assert_eq!(
        manifest
            .get("why_use_google_if_you_have_bing", "greatest_search_engine")
            .unwrap(),
        "Bing"
    );
    assert_eq!(
        manifest
            .get(
                "why_use_google_if_you_have_bing",
                "most_valiant_search_engine"
            )
            .unwrap(),
        "Bing"
    );
}

#[test]
fn test_with_comments() {
    let hardcoded_test = r#"
        # Imagine all the people
		# Living life in peace
		# Youhohoo
		# You may say I'm a dreamer
		# But I'm not the only one
		# I hope someday you'll join us
		# And the world will be as one
		# And the world, and the wooorld, and the world... Will be as one
        [mohylyanka]
        # Best discipline
        ipz = best
    "#;

    let manifest = Manifest::parse(hardcoded_test).unwrap();
    assert_eq!(manifest.get("mohylyanka", "ipz").unwrap(), "best");
}

#[test]
fn test_invalid_manifest() {
    let hardcoded_test = "is_c_popular = false";

    assert!(Manifest::parse(hardcoded_test).is_err());
}

#[test]
fn test_missing_section() {
    let hardcoded_test = r#"
        [mohylyanka]
        ipz = best
    "#;

    let manifest = Manifest::parse(hardcoded_test).unwrap();
    let result = manifest.get("us_president", "current");

    match result {
        Err(ManifestError::MissingSection(section)) => {
            assert_eq!(section, "us_president");
        }
        _ => panic!("Expected MissingSection error"),
    }
}

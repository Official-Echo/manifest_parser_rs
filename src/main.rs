use manifest_parser_rs::{Manifest, ManifestError};

fn main() {
    let hardcoded_test = r#"
        [some_section]
        some_key = some_value
		huh = 😁😁😁
    "#;

    match Manifest::parse(hardcoded_test) {
        Ok(manifest) => {
            let result = manifest.get("some_section", "huh");
            match result {
                Err(ManifestError::MissingSection(section)) => {
                    println!("Missing section: {}", section);
                }
                Ok(string) => println!("Result: {:?}", string),
				_ => {}
            }
        }
        Err(e) => {
            println!("Failed to parse manifest: {}", e);
		}
	}
}

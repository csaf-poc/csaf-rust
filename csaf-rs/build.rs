use std::path::Path;
use std::{fs, io};
use std::string::ToString;
use thiserror::Error;
use typify::{TypeSpace, TypeSpaceSettings};
use json_dotpath::DotPaths;
use serde_json::{json, Value};

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("JSON schema error")]
    SchemaError(#[from] typify::Error),
    #[error("Rust syntax error")]
    SyntaxError(#[from] syn::Error),
    #[error("JSON parsing error")]
    JsonError(#[from] serde_json::Error),
    #[error("other error")]
    Other,
}

fn main() -> Result<(), BuildError> {
    println!("cargo:rerun-if-changed=build.rs");

    // All schema files for change watching
    let schema_configs = [
        (
            "assets/csaf_2.0_json_schema.json",
            "csaf/csaf2_0/schema.rs",
            Some(&fix_2_0_schema as &dyn Fn(&mut Value)),
        ),
        (
            "assets/ssvc-1-0-1-merged.schema.json",
            "csaf/csaf2_1/ssvc_schema.rs",
            None,
        ),
        (
            "assets/csaf_2.1_json_schema.json",
            "csaf/csaf2_1/schema.rs",
            Some(&fix_2_1_schema),
        ),
        (
            "assets/decision_point_1.0.1_json_schema.json",
            "csaf/csaf2_1/ssvc_dp_schema.rs",
            None,
        )
    ];

    // Register watching for all inputs
    for (input, _, _) in &schema_configs {
        println!("cargo:rerun-if-changed={}", input);
    }

    // Execute all listed schema builds
    for (input, output, schema_patch) in &schema_configs {
        build(input, output, schema_patch)?;
    }

    generate_language_subtags()?;

    Ok(())
}

fn build(
    input: &str,
    output: &str,
    schema_patch: &Option<&dyn Fn(&mut Value)>
) -> Result<(), BuildError> {
    let content = fs::read_to_string(&input)?;
    let mut schema_value = serde_json::from_str(&content)?;
    // Execute a schema patch function, if provided.
    if let Some(patch_fn) = schema_patch {
        patch_fn(&mut schema_value);
    }
    let schema: schemars::schema::RootSchema = serde_json::from_value(schema_value)?;

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(true)
            .with_derive("PartialEq".into())
            .with_derive("Eq".into()),
    );
    type_space.add_root_schema(schema)?;

    let content = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);

    let mut out_file = Path::new("src").to_path_buf();
    out_file.push(output);
    Ok(fs::write(out_file, content)?)
}

/// Patches (unsupported) external schemas to the plain object type for CSAF 2.0.
fn fix_2_0_schema(value: &mut Value) {
    let prefix = "properties.vulnerabilities.items.properties.scores.items.properties";
    let fix_paths = [
        format!("{}.cvss_v2", prefix),
        format!("{}.cvss_v3", prefix),
    ];
    for path in fix_paths {
        value.dot_set(path.as_str(), json!({"type": "object"})).unwrap();
    }
    remove_datetime_formats(value);
}

/// Patches (unsupported) external schemas to the plain object type for CSAF 2.1.
fn fix_2_1_schema(value: &mut Value) {
    let prefix =
        "properties.vulnerabilities.items.properties.metrics.items.properties.content.properties";
    let fix_paths = [
        format!("{}.cvss_v2", prefix),
        format!("{}.cvss_v3", prefix),
        format!("{}.cvss_v4", prefix),
        format!("{}.ssvc_v1", prefix),
    ];
    for path in fix_paths {
        value.dot_set(path.as_str(), json!({"type": "object"})).unwrap();
    }
    remove_datetime_formats(value);
}

/// Recursively searches for "format": "date-time" and removes this format.
fn remove_datetime_formats(value: &mut Value) {
    if let Value::Object(map) = value {
        if let Some(format) = map.get("format") {
            if format.as_str() == Some("date-time") {
                // Remove the format property entirely
                map.remove("format");
            }
        }

        // Recursively process all values in the object
        for (_, v) in map.iter_mut() {
            remove_datetime_formats(v);
        }
    } else if let Value::Array(arr) = value {
        for item in arr.iter_mut() {
            remove_datetime_formats(item);
        }
    }
}

/// Compile-time-embedded language-subtag-registry.txt
const LANGUAGE_REGISTRY: &str = include_str!("assets/language-subtag-registry.txt");

/// Generates the language subtags array from the build-embedded text file.
fn generate_language_subtags() -> Result<(), BuildError> {
    let mut subtags = Vec::new();
    let mut current_entry_type = None;

    for line in LANGUAGE_REGISTRY.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("%%") {
            current_entry_type = None;
            continue;
        }

        if let Some(type_value) = line.strip_prefix("Type: ") {
            current_entry_type = Some(type_value.to_string());
            continue;
        }

        if let Some(ref entry_type) = current_entry_type {
            if entry_type == "language" {
                if let Some(subtag) = line.strip_prefix("Subtag: ") {
                    subtags.push(subtag.to_string());
                }
            }
        }
    }

    subtags.sort_unstable();

    let mut code = String::new();
    code.push_str("// Auto-generated by build.rs\n");
    code.push_str("pub static LANGUAGE_SUBTAGS_ARRAY: &[&str] = &[\n");

    for subtag in &subtags {
        code.push_str(&format!("    \"{}\",\n", subtag));
    }

    code.push_str("];\n\n");

    // Zusätzlich eine Lookup-Funktion generieren
    code.push_str("pub fn is_valid_language_subtag(subtag: &str) -> bool {\n");
    code.push_str("    LANGUAGE_SUBTAGS_ARRAY.binary_search(&subtag).is_ok()\n");
    code.push_str("}\n");

    let out_path = Path::new("src")
        .join("csaf")
        .join("generated")
        .join("language_subtags.rs");
    fs::write(&out_path, code)?;

    println!("cargo:rerun-if-changed=../assets/language-subtag-registry.txt");
    Ok(())
}

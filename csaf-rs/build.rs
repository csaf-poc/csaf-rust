use std::path::Path;
use std::{fs, io};
use thiserror::Error;
use typify::{TypeSpace, TypeSpaceSettings};

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
    // We only need to generate these files as part of our cargo build process,
    // not if we are publishing or getting built by cargo from a crates.io
    // package. This is because the files are generated from the JSON schema
    // files, which are not included in the published package.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    if manifest_dir.to_string_lossy().contains("target/package") {
        // If we're in target/package/<version>, we don't need to generate the files
        // because they are already generated in the package.
        return Ok(());
    } else if manifest_dir.to_string_lossy().contains("crates.io") {
        // If we're in a crates.io folder we don't need to generate the files
        // because they are already generated in the debug build.
        return Ok(());
    }

    build(
        "./src/csaf/csaf2_0/csaf_json_schema.json",
        "csaf/csaf2_0/schema.rs",
        true,
    )?;
    build(
        "./src/csaf/csaf2_1/ssvc-1-0-1-merged.schema.json",
        "csaf/csaf2_1/ssvc_schema.rs",
        false,
    )?;
    build(
        "./src/csaf/csaf2_1/csaf.json",
        "csaf/csaf2_1/schema.rs",
        true,
    )?;
    build(
        "../ssvc/data/schema/v1/Decision_Point-1-0-1.schema.json",
        "csaf/csaf2_1/ssvc_dp_schema.rs",
        false,
    )?;

    Ok(())
}

fn build(input: &str, output: &str, no_date_time: bool) -> Result<(), BuildError> {
    let content = fs::read_to_string(input)?;
    let mut schema_value = serde_json::from_str(&content)?;
    if no_date_time {
        // Recursively search for "format": "date-time" and remove this format
        remove_datetime_formats(&mut schema_value);
    }
    let schema: schemars::schema::RootSchema = serde_json::from_value(schema_value)?;

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(true)
            .with_derive("PartialEq".into())
            .with_derive("Eq".into()),
    );
    type_space.add_root_schema(schema)?;

    let mut content = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);
    content.insert_str(
        0,
        r#"
#![allow(clippy::clone_on_copy)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::len_zero)]
"#,
    );

    let mut out_file = Path::new("src").to_path_buf();
    out_file.push(output);
    Ok(fs::write(out_file, content)?)
}

fn remove_datetime_formats(value: &mut serde_json::Value) {
    if let serde_json::Value::Object(map) = value {
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
    } else if let serde_json::Value::Array(arr) = value {
        for item in arr.iter_mut() {
            remove_datetime_formats(item);
        }
    }
}

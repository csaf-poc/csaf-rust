use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Serialize)]
pub struct ValidationError {
    pub message: String,
    #[serde(rename = "instancePath")]
    pub instance_path: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValidationError: {} at {}",
            self.message, self.instance_path
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ValidationPreset {
    Basic,
    Extended,
    Full,
}

impl FromStr for ValidationPreset {
    type Err = ();

    fn from_str(input: &str) -> Result<ValidationPreset, Self::Err> {
        match input {
            "basic" => Ok(ValidationPreset::Basic),
            "extended" => Ok(ValidationPreset::Extended),
            "full" => Ok(ValidationPreset::Full),
            _ => Err(()),
        }
    }
}

impl Display for ValidationPreset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Extended => write!(f, "extended"),
            Self::Full => write!(f, "full"),
        }
    }
}

pub trait Validate {
    /// Validates this object according to a validation preset
    fn validate_preset(&'static self, preset: ValidationPreset);

    /// Validates this object according to a specific test ID.
    fn validate_by_test(&self, version: &str);
}

pub type Test<VersionedDocument> = fn(&VersionedDocument) -> Result<(), ValidationError>;

/// Represents something which is validatable according to the CSAF standard.
/// This trait MUST be implemented by the struct that represents a CSAF document
/// in the respective version.
///
/// It can then be used to validate documents with either [validate_by_preset] or [validate_by_test].
pub trait Validatable<VersionedDocument> {
    /// Returns a hashmap containing the test ID per preset
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>>;

    /// Returns a hashmap containing the test function per test ID
    fn tests(&self) -> HashMap<&str, Test<VersionedDocument>>;

    fn doc(&self) -> &VersionedDocument;
}

/// Executes all tests of the specified [preset] against the [target]
/// (which is of type [VersionedDocument], e.g. a CSAF 2.0 document).
pub fn validate_by_preset<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    preset: ValidationPreset,
) {
    println!("Validating document with {:?} preset... \n", preset);

    // Loop through tests
    if let Some(tests) = target.presets().get(&preset) {
        for test_id in tests {
            println!("Executing Test {}... ", test_id);
            validate_by_test(target, test_id);

            println!()
        }
    } else {
        println!("No tests found for preset")
    }
}

pub fn validate_by_test<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    test_id: &str,
) {
    if let Some(test_fn) = target.tests().get(test_id) {
        match test_fn(target.doc()) {
            Ok(()) => println!("> Test Success"),
            Err(e) => println!("> Error: {}", e),
        };
    } else {
        println!("Test with ID {} is missing implementation", test_id);
    }
}

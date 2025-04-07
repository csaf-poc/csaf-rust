use std::collections::HashMap;
use crate::csaf::csaf2_1::loader::load_document;
use crate::csaf::csaf2_1::schema::CommonSecurityAdvisoryFramework;
use crate::csaf::validation::{Test, ValidationError};

/// Generic test helper that loads all test files matching a specific test number pattern
/// and runs positive and negative validations against a test function.
///
/// # Arguments
/// * `test_number` - The test number to run (e.g., "36" for 6.1.36 tests)
/// * `test_function` - The test function to execute against each document
/// * `negative_cases` - A slice of tuples containing (file_suffix, expected_validation_error)
///                     for negative test cases (starting with "0")
///
/// This function assumes tests with filenames ending with numbers starting with "0"
/// are negative tests, and those starting with "1" are positive tests.
pub fn run_csaf21_tests(
    test_number: &str,
    test_function: Test<CommonSecurityAdvisoryFramework>,
    expected_errors: HashMap<&str, &ValidationError>,
) {
    use glob::glob;

    // Find all test files matching the pattern
    let pattern = &format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-{}-*.json", test_number);
    let file_prefix = &format!("oasis_csaf_tc-csaf_2_1-2024-6-1-{}-", test_number);

    // Load and test each file
    for entry in glob(pattern).expect("Failed to parse glob pattern") {
        if let Ok(path) = entry {
            // Extract the file suffix (e.g., "01", "02", etc.)
            let file_name = path.file_name().unwrap().to_string_lossy();
            let test_num = file_name
                .strip_prefix(file_prefix)
                .unwrap()
                .strip_suffix(".json")
                .unwrap();

            // Load the document
            let doc = load_document(path.to_string_lossy().as_ref()).unwrap();

            // Check if this is expected to be a negative or positive test case
            if test_num.starts_with('0') {
                // Negative test case - should fail with specific error
                let expected_error = expected_errors.get(test_num).expect(
                    &format!("Missing expected error definition for negative test case {}", test_num)
                );
                assert_eq!(
                    Err((*expected_error).clone()),
                    test_function(&doc),
                    "Negative test case {} should have failed with the expected error", test_num
                );
            } else if test_num.starts_with('1') {
                // Positive test case - should succeed
                assert_eq!(
                    Ok(()),
                    test_function(&doc),
                    "Positive test case {} should have succeeded", test_num
                );
            } else {
                panic!("Unexpected test case number format: {}", test_num);
            }
        }
    }
}
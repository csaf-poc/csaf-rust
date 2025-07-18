use crate::csaf::csaf2_0::loader::load_document as load_document_20;
use crate::csaf::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf20;
use crate::csaf::csaf2_1::loader::load_document as load_document_21;
use crate::csaf::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf21;
use crate::csaf::validation::{Test, ValidationError};
use std::collections::HashMap;

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
fn run_csaf_tests<CsafType>(
    pattern: &str,
    file_prefix: &str,
    document_loader: fn(&str) -> std::io::Result<CsafType>,
    test_function: Test<CsafType>,
    expected_errors: &HashMap<&str, &ValidationError>,
    skipped_tests: &[&str],
) {
    use glob::glob;

    // Load and test each file
    for entry in glob(pattern).expect("Failed to parse glob pattern") {
        if let Ok(path) = entry {
            // Extract the file suffix (e.g., "01", "02", etc.)
            let file_name = path.file_name().unwrap().to_string_lossy();
            println!("{}", file_name);
            let test_num = file_name
                .strip_prefix(file_prefix)
                .unwrap()
                .strip_suffix(".json")
                .unwrap();

            if skipped_tests.contains(&test_num) {
                println!("Skipping test {}", test_num);
                continue;
            }

            // Load the document
            let doc = document_loader(path.to_string_lossy().as_ref()).unwrap();

            // Check if this is expected to be a negative or positive test case
            if test_num.starts_with('0') || test_num.starts_with('2') {
                // Negative test case - should fail with a specific error
                let expected_error = expected_errors
                    .get(test_num)
                    .expect(
                        &format!(
                            "Missing expected error definition for negative test case {}",
                            test_num
                        )
                    );
                assert_eq!(
                    Err((*expected_error).clone()),
                    test_function(&doc),
                    "Negative test case {} should have failed with the expected error",
                    test_num
                );
            } else if test_num.starts_with('1') {
                // Positive test case - should succeed
                assert_eq!(
                    Ok(()),
                    test_function(&doc),
                    "Positive test case {} should have succeeded",
                    test_num
                );
            } else {
                panic!("Unexpected test case number format: {}", test_num);
            }
        }
    }
}

pub fn run_csaf20_tests_with_excludes(
    test_number: &str,
    test_function: Test<Csaf20>,
    expected_errors: &HashMap<&str, &ValidationError>,
    skipped_tests: &[&str],
) {
    // Find all test files matching the pattern
    let file_prefix = &format!("oasis_csaf_tc-csaf_2_0-2021-6-1-{}-", test_number);
    let pattern = &format!(
        "assets/csaf/csaf_2.0/test/validator/data/mandatory/{}*.json",
        file_prefix
    );

    run_csaf_tests(
        pattern,
        file_prefix,
        load_document_20,
        test_function,
        expected_errors,
        skipped_tests
    );
}

pub fn run_csaf21_tests_with_excludes(
    test_number: &str,
    test_function: Test<Csaf21>,
    expected_errors: &HashMap<&str, &ValidationError>,
    skipped_tests: &[&str],
) {
    // Find all test files matching the pattern
    let file_prefix = &format!("oasis_csaf_tc-csaf_2_1-2024-6-1-{}-", test_number);
    let pattern = &format!(
        "assets/csaf/csaf_2.1/test/validator/data/mandatory/{}*.json",
        file_prefix
    );

    run_csaf_tests(
        pattern,
        file_prefix,
        load_document_21,
        test_function,
        expected_errors,
        skipped_tests
    );
}

/// Overload for run_csaf20_tests without the skipped_tests parameter
pub fn run_csaf20_tests(
    test_number: &str,
    test_function: Test<Csaf20>,
    expected_errors: &HashMap<&str, &ValidationError>,
) {
    run_csaf20_tests_with_excludes(test_number, test_function, expected_errors, &[])
}

/// Overload for run_csaf21_tests without the skipped_tests parameter
pub fn run_csaf21_tests(
    test_number: &str,
    test_function: Test<Csaf21>,
    expected_errors: &HashMap<&str, &ValidationError>,
) {
    run_csaf21_tests_with_excludes(test_number, test_function, expected_errors, &[])
}

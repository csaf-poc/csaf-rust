use crate::csaf::getter_traits::CsafTrait;
use crate::csaf::product_helpers::gather_product_definitions;
use crate::csaf::validation::ValidationError;
use std::collections::HashMap;

pub fn test_6_1_02_multiple_definition_of_product_id(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let definitions: Vec<_> = gather_product_definitions(doc);
    let duplicates = find_duplicates(definitions);

    if let Some(duplicate) = duplicates.first() {
        Err(ValidationError {
            message: format!("Duplicate definition for product ID {}", duplicate.0),
            instance_path: duplicate.1[1].to_owned(),
        })
    } else {
        Ok(())
    }
}

fn find_duplicates(vec: Vec<(String, String)>) -> Vec<(String, Vec<String>)> {
    // Map to store each key with all of its paths
    let mut conflicts = HashMap::new();

    for (key, path) in vec {
        // Add this path to the list for this key
        conflicts.entry(key).or_insert_with(Vec::new).push(path);
    }

    // Filter to keep only entries with multiple paths (actual duplicates)
    conflicts.into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::csaf::csaf2_0::loader::load_document as load_20;
    use crate::csaf::csaf2_1::loader::load_document as load_21;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_02::test_6_1_02_multiple_definition_of_product_id;

    static EXPECTED_ERROR: &str = "Duplicate definition for product ID CSAFPID-9080700";
    static EXPECTED_INSTANCE_PATH: &str = "/product_tree/full_product_names/1/product_id";

    #[test]
    fn test_test_6_1_02_csaf_2_0() {
        let doc = load_20("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-02-01.json").unwrap();
        assert_eq!(
            test_6_1_02_multiple_definition_of_product_id(&doc),
            Err(ValidationError {
                message: EXPECTED_ERROR.to_string(),
                instance_path: EXPECTED_INSTANCE_PATH.to_string(),
            })
        )
    }

    #[test]
    fn test_test_6_1_02_csaf_2_1() {
        let doc = load_21("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-02-01.json").unwrap();
        assert_eq!(
            test_6_1_02_multiple_definition_of_product_id(&doc),
            Err(ValidationError {
                message: EXPECTED_ERROR.to_string(),
                instance_path: EXPECTED_INSTANCE_PATH.to_string(),
            })
        )
    }
}

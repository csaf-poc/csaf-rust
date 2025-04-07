use crate::csaf::getter_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::validation::ValidationError;
use std::collections::HashSet;

pub fn test_6_1_02_multiple_definition_of_product_id(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    // Map to store each key with all of its paths
    let mut conflicts = HashSet::<String>::new();

    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |product, path| {
            if conflicts.contains(product.get_product_id()) {
                Err(ValidationError {
                    message: format!("Duplicate definition for product ID {}", product.get_product_id()),
                    instance_path: format!("{}/product_id", path),
                })
            } else {
                conflicts.insert(product.get_product_id().to_owned());
                Ok(())
            }
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::csaf::csaf2_0::loader::load_document as load_20;
    use crate::csaf::test_helper::run_csaf21_tests;
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
        run_csaf21_tests("02", test_6_1_02_multiple_definition_of_product_id, HashMap::from([
            ("01", &ValidationError {
                message: EXPECTED_ERROR.to_string(),
                instance_path: EXPECTED_INSTANCE_PATH.to_string(),
            })
        ]));
    }
}

use crate::csaf::getter_traits::CsafTrait;
use crate::csaf::product_helpers::{gather_product_definitions, gather_product_references};
use std::collections::HashSet;
use crate::csaf::validation::ValidationError;

pub fn test_6_1_01_missing_definition_of_product_id(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let definitions = gather_product_definitions(doc);
    let definitions_set = HashSet::<String>::from_iter(definitions.iter().map(|x| x.1.to_owned()));
    let references = gather_product_references(doc);

    for (ref_id, ref_path) in references.iter() {
        if !definitions_set.contains(ref_id) {
            return Err(ValidationError {
                message: format!("Missing definition of product_id: {}", ref_id),
                instance_path: ref_path.to_string(),
            })
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::csaf2_0::loader::load_document as load_20;
    use crate::csaf::csaf2_1::loader::load_document as load_21;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_01::test_6_1_01_missing_definition_of_product_id;

    static EXPECTED_ERROR: &str = "Missing definition of product_id: CSAFPID-9080700";
    static EXPECTED_INSTANCE_PATH: &str = "/product_tree/product_groups/0/product_ids/0";

    #[test]
    fn test_6_1_01_csaf_2_0() {
        let doc = load_20("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-01.json").unwrap();
        assert_eq!(
            test_6_1_01_missing_definition_of_product_id(&doc),
            Err(ValidationError {
                message: EXPECTED_ERROR.to_string(),
                instance_path: EXPECTED_INSTANCE_PATH.to_string(),
            })
        );
    }

    #[test]
    fn test_6_1_01_csaf_2_1() {
        let doc = load_21("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-01-01.json").unwrap();
        assert_eq!(
            test_6_1_01_missing_definition_of_product_id(&doc),
            Err(ValidationError {
                message: EXPECTED_ERROR.to_string(),
                instance_path: EXPECTED_INSTANCE_PATH.to_string(),
            })
        );
    }
}

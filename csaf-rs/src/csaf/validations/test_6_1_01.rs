use crate::csaf::getter_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::product_helpers::gather_product_references;
use std::collections::HashSet;
use crate::csaf::validation::ValidationError;

pub fn test_6_1_01_missing_definition_of_product_id(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let mut definitions_set = HashSet::<String>::new();
    if let Some(tree) = doc.get_product_tree().as_ref() {
        _ = tree.visit_all_products(&mut |fpn, _path| {
            definitions_set.insert(fpn.get_product_id().to_owned());
            Ok(())
        });
    }

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
    use std::collections::HashMap;
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_01::test_6_1_01_missing_definition_of_product_id;

    #[test]
    fn test_test_6_1_01() {
        let error01 = ValidationError {
            message: "Missing definition of product_id: CSAFPID-9080700".to_string(),
            instance_path: "/product_tree/product_groups/0/product_ids/0".to_string(),
        };
        let error02 = ValidationError {
            message: "Missing definition of product_id: CSAFPID-9080701".to_string(),
            instance_path: "/vulnerabilities/0/flags/0/product_ids/1".to_string(),
        };
        let errors = &HashMap::from([
            ("01", &error01),
            ("02", &error02),
        ]);
        run_csaf20_tests("01", test_6_1_01_missing_definition_of_product_id, &errors);
        run_csaf21_tests("01", test_6_1_01_missing_definition_of_product_id, &errors);
    }
}

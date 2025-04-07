use crate::csaf::getter_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::helpers::count_unescaped_stars;
use crate::csaf::validation::ValidationError;

pub fn test_6_1_43_multiple_stars_in_model_number(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(model_numbers) = helper.get_model_numbers() {
                    for (index, model_number) in model_numbers.enumerate() {
                        if count_unescaped_stars(model_number) > 1 {
                            return Err(ValidationError {
                                message: "Model number must not contain multiple unescaped asterisks (stars)".to_string(),
                                instance_path: format!("{}/product_identification_helper/model_numbers/{}", path, index),
                            });
                        }
                    }
                }
            }
            Ok(())
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_43::test_6_1_43_multiple_stars_in_model_number;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_43() {
        let expected_error = ValidationError {
            message: "Model number must not contain multiple unescaped asterisks (stars)".to_string(),
            instance_path: "/product_tree/full_product_names/0/product_identification_helper/model_numbers/0".to_string(),
        };

        run_csaf21_tests(
            "43",
            test_6_1_43_multiple_stars_in_model_number, HashMap::from([
                ("01", &expected_error),
                ("02", &expected_error),
            ])
        );
    }
}

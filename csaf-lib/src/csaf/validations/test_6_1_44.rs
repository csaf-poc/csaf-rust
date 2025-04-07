use crate::csaf::getter_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::helpers::count_unescaped_stars;
use crate::csaf::validation::ValidationError;

pub fn test_6_1_44_multiple_stars_in_serial_number(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(serial_numbers) = helper.get_serial_numbers() {
                    for (index, serial_number) in serial_numbers.enumerate() {
                        if count_unescaped_stars(serial_number) > 1 {
                            return Err(ValidationError {
                                message: "Serial number must not contain multiple unescaped asterisks (stars)".to_string(),
                                instance_path: format!("{}/product_identification_helper/serial_numbers/{}", path, index),
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
    use std::collections::HashMap;
    use crate::csaf::validations::test_6_1_44::test_6_1_44_multiple_stars_in_serial_number;

    #[test]
    fn test_test_6_1_44() {
        let expected_error = ValidationError {
            message: "Serial number must not contain multiple unescaped asterisks (stars)".to_string(),
            instance_path: "/product_tree/full_product_names/0/product_identification_helper/serial_numbers/0".to_string(),
        };

        run_csaf21_tests(
            "44",
            test_6_1_44_multiple_stars_in_serial_number, HashMap::from([
                ("01", &expected_error),
                ("02", &expected_error),
            ])
        );
    }
}

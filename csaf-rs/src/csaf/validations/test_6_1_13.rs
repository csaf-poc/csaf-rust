use crate::csaf::getter_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::validation::ValidationError;
use packageurl::PackageUrl;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static PURL_REGEX: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"^pkg:[A-Za-z.\-+][A-Za-z0-9.\-+]*/.+").unwrap()
);

pub fn test_6_1_13_purl(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    for (i, purl_str) in purls.iter().enumerate() {
                        // Check against PURL regex
                        if !PURL_REGEX.is_match(purl_str) {
                            return Err(ValidationError {
                                message: format!("PURL doesn't comply with CSAF PURL regex: {}", purl_str),
                                instance_path: format!("{}/product_identification_helper/purls/{}", path, i),
                            });
                        }
                        // Parse the PURL
                        match PackageUrl::from_str(purl_str) {
                            Ok(_) => {},
                            Err(_) => {
                                return Err(ValidationError {
                                    message: format!("Invalid PURL format: {}", purl_str),
                                    instance_path: format!("{}/product_identification_helper/purls/{}", path, i),
                                });
                            }
                        };
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
    use crate::csaf::test_helper::{run_csaf20_tests_with_excludes, run_csaf21_tests_with_excludes};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_13::test_6_1_13_purl;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_13() {
        let error01 = ValidationError {
            message: "Invalid PURL format: pkg:maven/@1.3.4".to_string(),
            instance_path: "/product_tree/full_product_names/0/product_identification_helper/purls/0".to_string(),
        };
        let error02 = ValidationError {
            message: "`packageurl` does not know about specifics of OCI type etc.".to_string(),
            instance_path: "dummy".to_string(),
        };
        let errors = HashMap::from([
            ("01", &error01),
            ("02", &error02),
        ]);
        run_csaf20_tests_with_excludes("13", test_6_1_13_purl, &errors, &["02"]);
        run_csaf21_tests_with_excludes("13", test_6_1_13_purl, &errors, &["02"]);
    }
}
use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::getter_traits::{CsafTrait, ProductStatusTrait, RemediationTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use std::collections::HashSet;

/// Remediation categories that conflict with the product status "not affected".
const NOT_AFFECTED_CONFLICTS: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::Workaround,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::NoneAvailable,
];

/// Remediation categories that conflict with "fixed" product statuses.
const FIXED_CONFLICTS: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::NoFixPlanned,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::Workaround,
];

pub fn test_6_1_36_status_group_contradicting_remediation_categories(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = v.get_product_status() {
            // Collect Product IDs that may cause conflicts
            let affected_products = product_status.get_all_affected();
            let not_affected_products = match product_status.get_known_not_affected() {
                Some(products) => products.into_iter().collect(),
                None => HashSet::new(),
            };
            let fixed_products = product_status.get_all_fixed();
            // Iterate over remediations
            for (r_i, r) in v.get_remediations().iter().enumerate() {
                // Only handle Remediations having product IDs associated
                if let Some(product_ids) = r.get_all_product_ids(doc) {
                    // Category of current remediation
                    let cat = r.get_category();
                    // Iterate over product IDs
                    for p in product_ids {
                        if affected_products.contains(&p) && cat == CategoryOfTheRemediation::OptionalPatch {
                            return Err(ValidationError {
                                message: format!(
                                    "Product {} is listed as affected but has conflicting remediation category {}",
                                    p,
                                    cat
                                ),
                                instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
                            });
                        }
                        if not_affected_products.contains(&p) && NOT_AFFECTED_CONFLICTS.contains(&cat) {
                            return Err(ValidationError {
                                message: format!(
                                    "Product {} is listed as not affected but has conflicting remediation category {}",
                                    p,
                                    cat
                                ),
                                instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
                            });
                        }
                        if fixed_products.contains(&p) && FIXED_CONFLICTS.contains(&cat) {
                            return Err(ValidationError {
                                message: format!(
                                    "Product {} is listed as fixed but has conflicting remediation category {}",
                                    p,
                                    cat
                                ),
                                instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_36::test_6_1_36_status_group_contradicting_remediation_categories;

    #[test]
    fn test_test_6_1_36() {
        run_csaf21_tests(
            "36",
            test_6_1_36_status_group_contradicting_remediation_categories,
            HashMap::from([
                ("01", &ValidationError {
                    message: "Product CSAFPID-9080700 is listed as not affected but has conflicting remediation category vendor_fix".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/0".to_string()
                }),
                ("02", &ValidationError {
                    message: "Product CSAFPID-9080703 is listed as fixed but has conflicting remediation category none_available".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/0".to_string()
                }),
                ("03", &ValidationError {
                    message: "Product CSAFPID-9080700 is listed as affected but has conflicting remediation category optional_patch".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/0".to_string(),
                }),
            ]),
        );
    }
}

use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::getter_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use std::collections::BTreeMap;

/// Totally exclusive categories that cannot be combined with any other category.
static EX_STATES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::OptionalPatch,
];

/// Mutually exclusive states that cannot apply at the same time.
static MUT_EX_STATES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoFixPlanned,
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::VendorFix,
];

pub fn test_6_1_35_contradicting_remediations(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        // Data struct to store observed remediation categories per product IT
        let mut product_categories: BTreeMap<String, Vec<CategoryOfTheRemediation>> = BTreeMap::new();
        for (r_i, r) in v.get_remediations().iter().enumerate() {
            // Only handle Remediations having product IDs associated
            if let Some(product_ids) = r.get_all_product_ids(doc) {
                // Category of current remediation
                let cat = r.get_category();
                // Iterate over product IDs
                for p in product_ids {
                    // Check if product ID has categories associated
                    if let Some(exist_cat_set) = product_categories.get_mut(&p) {
                        // Checks if current category is exclusive and a non-equal previous category was found.
                        if EX_STATES.contains(&cat) && exist_cat_set.first().is_some_and(|e_cat| e_cat != &cat)
                            // Checks if the (only) previous category is exclusive.
                            || exist_cat_set.first().is_some_and(|e_cat| EX_STATES.contains(e_cat))
                            // Checks if the current category conflicts with any other in the group of mutually exclusive ones.
                            || MUT_EX_STATES.contains(&cat) && exist_cat_set.iter().any(|e_cat| MUT_EX_STATES.contains(e_cat))
                        {
                            return Err(ValidationError {
                                message: format!(
                                    "Product {} has contradicting remediations: {} and {}",
                                    p,
                                    exist_cat_set.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "),
                                    cat
                                ),
                                instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
                            });
                        }
                        exist_cat_set.push(cat);
                    } else {
                        product_categories.insert(p, Vec::from([cat]));
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_35::test_6_1_35_contradicting_remediations;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_35() {
        run_csaf21_tests(
            "35",
            test_6_1_35_contradicting_remediations,
            HashMap::from([
                ("01", &ValidationError {
                    message: "Product CSAFPID-9080700 has contradicting remediations: no_fix_planned and vendor_fix".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/1".to_string(),
                }),
                ("02", &ValidationError {
                    message: "Product CSAFPID-9080700 has contradicting remediations: none_available and mitigation".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/1".to_string(),
                }),
                ("03", &ValidationError {
                    message: "Product CSAFPID-9080702 has contradicting remediations: workaround, fix_planned and optional_patch".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/2".to_string(),
                }),
                ("04", &ValidationError {
                    message: "Product CSAFPID-9080701 has contradicting remediations: mitigation, fix_planned and optional_patch".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/2".to_string(),
                }),
            ]),
        );
    }
}

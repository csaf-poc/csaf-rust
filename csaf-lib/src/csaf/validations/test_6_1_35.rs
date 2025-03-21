use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::getter_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};
use std::collections::BTreeMap;
use crate::csaf::validation::ValidationError;

static MUT_EX_MEASURES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::Workaround,
    CategoryOfTheRemediation::Mitigation,
];

static MUT_EX_FIX_STATES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::NoFixPlanned,
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::OptionalPatch,
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
                        // Check if any seen category conflicts with the current one
                        if exist_cat_set.iter().any(|e_cat| {
                            MUT_EX_MEASURES.contains(e_cat) && MUT_EX_MEASURES.contains(&cat)
                                || MUT_EX_FIX_STATES.contains(e_cat) && MUT_EX_FIX_STATES.contains(&cat)
                        }) {
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
    use crate::csaf::csaf2_1::loader::load_document;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_35::test_6_1_35_contradicting_remediations;

    #[test]
    fn test_test_6_1_35() {
        for x in ["11", "12", "13", "14"].iter() {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-35-{}.json", x).as_str()).unwrap();
            assert_eq!(
                Ok(()),
                test_6_1_35_contradicting_remediations(&doc)
            )
        }
        for (x, err) in [
            ("01", ValidationError {
                message: "Product CSAFPID-9080700 has contradicting remediations: no_fix_planned and vendor_fix".to_string(),
                instance_path: "/vulnerabilities/0/remediations/1".to_string()
            }),
            ("02", ValidationError {
                message: "Product CSAFPID-9080700 has contradicting remediations: none_available and mitigation".to_string(),
                instance_path: "/vulnerabilities/0/remediations/1".to_string()
            }),
            ("03", ValidationError {
                message: "Product CSAFPID-9080702 has contradicting remediations: workaround, fix_planned and optional_patch".to_string(),
                instance_path: "/vulnerabilities/0/remediations/2".to_string(),
            }),
            ("04", ValidationError {
                message: "Product CSAFPID-9080701 has contradicting remediations: mitigation, fix_planned and optional_patch".to_string(),
                instance_path: "/vulnerabilities/0/remediations/2".to_string(),
            }),
        ].iter() {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-35-{}.json", x).as_str()).unwrap();
            assert_eq!(
                Err(err.clone()),
                test_6_1_35_contradicting_remediations(&doc)
            )
        }
    }
}

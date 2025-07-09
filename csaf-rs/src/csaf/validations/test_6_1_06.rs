use crate::csaf::getter_traits::{CsafTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Contradiction Product Status Groups
#[derive(PartialEq, Clone)]
enum ProductStatusGroup {
    // first_affected, known_affected, last_affected
    Affected,
    // known_not_affected
    NotAffected,
    // first_fixed, fixed
    Fixed,
    // under_investigation
    UnderInvestigation,
    // unknown
    Unknown,
}

impl Display for ProductStatusGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductStatusGroup::Affected => write!(f, "affected"),
            ProductStatusGroup::NotAffected => write!(f, "not affected"),
            ProductStatusGroup::Fixed => write!(f, "fixed"),
            ProductStatusGroup::UnderInvestigation => write!(f, "under investigation"),
            ProductStatusGroup::Unknown => write!(f, "unknown"),
        }
    }
}

pub fn test_6_1_06_contradicting_product_status(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = v.get_product_status() {
            // Map of product IDs to product status groups (mutually exclusive, therefore only one allowed)
            let mut product_statuses: HashMap<String, ProductStatusGroup> = HashMap::new();

            // Handle all products with an "affected" status - these don't need conflict checking
            for pid in product_status.get_all_affected() {
                product_statuses.insert(pid.to_owned(), ProductStatusGroup::Affected);
            }

            // Handle all other status groups with conflict checking
            check_status_group(
                v_i,
                &mut product_statuses,
                product_status.get_known_not_affected(),
                ProductStatusGroup::NotAffected,
                "known_not_affected",
            )?;

            check_status_group(
                v_i,
                &mut product_statuses,
                product_status.get_first_fixed(),
                ProductStatusGroup::Fixed,
                "first_fixed",
            )?;
            check_status_group(
                v_i,
                &mut product_statuses,
                product_status.get_fixed(),
                ProductStatusGroup::Fixed,
                "fixed",
            )?;

            check_status_group(
                v_i,
                &mut product_statuses,
                product_status.get_under_investigation(),
                ProductStatusGroup::UnderInvestigation,
                "under_investigation",
            )?;

            check_status_group(
                v_i,
                &mut product_statuses,
                product_status.get_unknown(),
                ProductStatusGroup::Unknown,
                "unknown",
            )?;
        }
    }
    Ok(())
}

// Helper function to check for status group conflicts
fn check_status_group<'a>(
    v_i: usize,
    product_statuses: &mut HashMap<String, ProductStatusGroup>,
    product_ids: Option<impl IntoIterator<Item = &'a String>>,
    status_group: ProductStatusGroup,
    field_name: &str,
) -> Result<(), ValidationError> {
    if let Some(products) = product_ids {
        for (i_pid, pid) in products.into_iter().enumerate() {
            match product_statuses.get(pid) {
                None => {
                    product_statuses.insert(pid.to_owned(), status_group.clone());
                }
                Some(existing_status) => {
                    if *existing_status != status_group {
                        return Err(ValidationError {
                            message: format!(
                                "Product {} is marked with product status group \"{}\" but has conflicting product status belonging to group \"{}\"",
                                pid,
                                status_group,
                                *existing_status
                            ),
                            instance_path: format!("/vulnerabilities/{}/product_status/{}/{}", v_i, field_name, i_pid),
                        });
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
    use crate::csaf::validations::test_6_1_06::test_6_1_06_contradicting_product_status;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_06() {
        let first_error_message = "Product CSAFPID-9080700 is marked with product status group \"not affected\" but has conflicting product status belonging to group \"affected\"";
        let first_error_path = "/vulnerabilities/0/product_status/known_not_affected/0";
        run_csaf21_tests(
            "06",
            test_6_1_06_contradicting_product_status,
            &HashMap::from([
                ("01", &ValidationError {
                    message: first_error_message.to_string(),
                    instance_path: first_error_path.to_string()
                }),
                ("02", &ValidationError {
                    message: first_error_message.to_string(),
                    instance_path: first_error_path.to_string()
                }),
                ("03", &ValidationError {
                    message: first_error_message.to_string(),
                    instance_path: first_error_path.to_string()
                }),
                ("04", &ValidationError {
                    message: "Product CSAFPID-9080701 is marked with product status group \"fixed\" but has conflicting product status belonging to group \"not affected\"".to_string(),
                    instance_path: "/vulnerabilities/0/product_status/fixed/0".to_string(),
                }),
                ("05", &ValidationError {
                    message: "Product CSAFPID-9080702 is marked with product status group \"fixed\" but has conflicting product status belonging to group \"affected\"".to_string(),
                    instance_path: "/vulnerabilities/0/product_status/first_fixed/0".to_string(),
                }),
                ("06", &ValidationError {
                    message: "Product CSAFPID-9080700 is marked with product status group \"unknown\" but has conflicting product status belonging to group \"affected\"".to_string(),
                    instance_path: "/vulnerabilities/0/product_status/unknown/0".to_string(),
                }),
            ]),
        );
    }
}

use crate::csaf::getter_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait};
use crate::csaf::validation::ValidationError;

static NAME_PUBLIC: &str = "Public";
static NAME_PRIVATE: &str = "No sharing allowed";
static MAX_UUID: &str = "ffffffff-ffff-ffff-ffff-ffffffffffff";
static NIL_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// Validates that a CSAF document with specific sharing group IDs has the correct corresponding name.
///
/// This function ensures that:
/// - When using the maximum UUID ("ffffffff-ffff-ffff-ffff-ffffffffffff"), the sharing group name
///   must be "Public".
/// - When using the nil UUID ("00000000-0000-0000-0000-000000000000"), the sharing group name
///   must be "No sharing allowed".
///
/// # Arguments
///
/// * `doc` - A reference to an object implementing the `CsafTrait` interface.
///
/// # Returns
///
/// * `Ok(())` if the validation passes.
/// * `Err(ValidationError)` if the validation fails, with a message explaining the reason
///   and the JSON path to the invalid element.
pub fn test_6_1_41_missing_sharing_group_name(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let distribution = doc.get_document().get_distribution_21()?;

    if let Some(sharing_group) = distribution.get_sharing_group() {
        // Check if max UUID is used
        if sharing_group.get_id() == MAX_UUID {
            // If max UUID is used, name must exist and be NAME_PUBLIC
            match sharing_group.get_name() {
                Some(name) if name == NAME_PUBLIC => {},
                _ => return Err(ValidationError {
                    message: format!("Max UUID requires sharing group name to be \"{}\".", NAME_PUBLIC),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                })
            }
        }
        // Check if nil UUID is used
        else if sharing_group.get_id() == NIL_UUID {
            // If nil UUID is used, name must exist and be NAME_PRIVATE
            match sharing_group.get_name() {
                Some(name) if name == NAME_PRIVATE => {},
                _ => return Err(ValidationError {
                    message: format!("Nil UUID requires sharing group name to be \"{}\".", NAME_PRIVATE),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                })
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
    use crate::csaf::validations::test_6_1_41::{test_6_1_41_missing_sharing_group_name, NAME_PRIVATE, NAME_PUBLIC};

    #[test]
    fn test_test_6_1_41() {
        run_csaf21_tests(
            "41",
            test_6_1_41_missing_sharing_group_name, HashMap::from([
                ("01", &ValidationError {
                    message: format!("Max UUID requires sharing group name to be \"{}\".", NAME_PUBLIC),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
                ("02", &ValidationError {
                    message: format!("Nil UUID requires sharing group name to be \"{}\".", NAME_PRIVATE),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
                ("03", &ValidationError {
                    message: format!("Max UUID requires sharing group name to be \"{}\".", NAME_PUBLIC),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
                ("04", &ValidationError {
                    message: format!("Nil UUID requires sharing group name to be \"{}\".", NAME_PRIVATE),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
            ])
        );
    }
}

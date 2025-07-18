use crate::csaf::getter_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait};
use crate::csaf::helpers::{MAX_UUID, NIL_UUID, SG_NAME_PRIVATE, SG_NAME_PUBLIC};
use crate::csaf::validation::ValidationError;

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
            // If max UUID is used, the name must exist and be NAME_PUBLIC
            match sharing_group.get_name() {
                Some(name) if name == SG_NAME_PUBLIC => {},
                _ => return Err(ValidationError {
                    message: format!("Max UUID requires sharing group name to be \"{}\".", SG_NAME_PUBLIC),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                })
            }
        }
        // Check if nil UUID is used
        else if sharing_group.get_id() == NIL_UUID {
            // If nil UUID is used, the name must exist and be NAME_PRIVATE
            match sharing_group.get_name() {
                Some(name) if name == SG_NAME_PRIVATE => {},
                _ => return Err(ValidationError {
                    message: format!("Nil UUID requires sharing group name to be \"{}\".", SG_NAME_PRIVATE),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                })
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_41::{test_6_1_41_missing_sharing_group_name, SG_NAME_PRIVATE, SG_NAME_PUBLIC};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_41() {
        run_csaf21_tests(
            "41",
            test_6_1_41_missing_sharing_group_name, &HashMap::from([
                ("01", &ValidationError {
                    message: format!("Max UUID requires sharing group name to be \"{}\".", SG_NAME_PUBLIC),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
                ("02", &ValidationError {
                    message: format!("Nil UUID requires sharing group name to be \"{}\".", SG_NAME_PRIVATE),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
                ("03", &ValidationError {
                    message: format!("Max UUID requires sharing group name to be \"{}\".", SG_NAME_PUBLIC),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
                ("04", &ValidationError {
                    message: format!("Nil UUID requires sharing group name to be \"{}\".", SG_NAME_PRIVATE),
                    instance_path: "/document/distribution/sharing_group/name".to_string()
                }),
            ])
        );
    }
}

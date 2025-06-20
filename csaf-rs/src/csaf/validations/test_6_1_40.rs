use crate::csaf::getter_traits::{CsafTrait, DistributionTrait, DocumentTrait, SharingGroupTrait};
use crate::csaf::validation::ValidationError;

static NAME_PUBLIC: &str = "Public";
static NAME_PRIVATE: &str = "No sharing allowed";
static MAX_UUID: &str = "ffffffff-ffff-ffff-ffff-ffffffffffff";
static NIL_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// Validates the sharing group name and ID combinations in a CSAF document.
///
/// This function checks if the sharing group name and ID in the document's distribution
/// follow specific rules:
///
/// - If the sharing group name is "Public", the ID must be the maximum UUID
///   ("ffffffff-ffff-ffff-ffff-ffffffffffff").
/// - If the sharing group name is "No sharing allowed", the ID must be the nil UUID
///   ("00000000-0000-0000-0000-000000000000").
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
#[allow(clippy::collapsible_if)]
pub fn test_6_1_40_invalid_sharing_group_name(doc: &impl CsafTrait) -> Result<(), ValidationError> {
    let distribution = doc.get_document().get_distribution_21()?;

    if let Some(sharing_group) = distribution.get_sharing_group() {
        if let Some(sharing_group_name) = sharing_group.get_name() {
            if sharing_group_name == NAME_PUBLIC {
                if sharing_group.get_id() != MAX_UUID {
                    return Err(ValidationError {
                        message: format!(
                            "Sharing group name \"{}\" is prohibited without max UUID.",
                            NAME_PUBLIC
                        ),
                        instance_path: "/document/distribution/sharing_group/name".to_string(),
                    });
                }
            } else if sharing_group_name == NAME_PRIVATE {
                if sharing_group.get_id() != NIL_UUID {
                    return Err(ValidationError {
                        message: format!(
                            "Sharing group name \"{}\" is prohibited without nil UUID.",
                            NAME_PRIVATE
                        ),
                        instance_path: "/document/distribution/sharing_group/name".to_string(),
                    });
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
    use crate::csaf::validations::test_6_1_40::{
        test_6_1_40_invalid_sharing_group_name, NAME_PRIVATE, NAME_PUBLIC,
    };
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_40() {
        run_csaf21_tests(
            "40",
            test_6_1_40_invalid_sharing_group_name,
            &HashMap::from([
                (
                    "01",
                    &ValidationError {
                        message: format!(
                            "Sharing group name \"{}\" is prohibited without max UUID.",
                            NAME_PUBLIC
                        ),
                        instance_path: "/document/distribution/sharing_group/name".to_string(),
                    },
                ),
                (
                    "02",
                    &ValidationError {
                        message: format!(
                            "Sharing group name \"{}\" is prohibited without nil UUID.",
                            NAME_PRIVATE
                        ),
                        instance_path: "/document/distribution/sharing_group/name".to_string(),
                    },
                ),
            ]),
        );
    }
}

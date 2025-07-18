use crate::csaf::getter_traits::{CsafTrait, DocumentTrait, ProductGroupTrait, ProductTreeTrait, VulnerabilityTrait, WithGroupIds};
use crate::csaf::validation::ValidationError;
use std::collections::HashSet;

pub fn test_6_1_04_missing_definition_of_product_group_id(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    if let Some(tree) = doc.get_product_tree().as_ref() {
        let mut known_groups = HashSet::<String>::new();
        // Collect all known product group IDs
        for g in tree.get_product_groups().iter() {
            known_groups.insert(g.get_group_id().to_owned());
        }

        // Check document notes
        if let Some(notes) = doc.get_document().get_notes() {
            for (i_n, note) in notes.iter().enumerate() {
                if let Some(group_ids) = note.get_group_ids() {
                    for (i_g, group_id) in group_ids.enumerate() {
                        if !known_groups.contains(group_id) {
                            return Err(ValidationError {
                                message: format!("Missing definition of product_group_id: {}", group_id),
                                instance_path: format!("/document/notes/{}/group_ids/{}", i_n, i_g),
                            });
                        }
                    }
                }
            }
        }

        // Check vulnerabilities
        for (i_v, vuln) in doc.get_vulnerabilities().iter().enumerate() {
            // Check vulnerability flags
            if let Some(flags) = vuln.get_flags() {
                for (i_f, flag) in flags.iter().enumerate() {
                    if let Some(group_ids) = flag.get_group_ids() {
                        for (i_g, group_id) in group_ids.enumerate() {
                            if !known_groups.contains(group_id) {
                                return Err(ValidationError {
                                    message: format!("Missing definition of product_group_id: {}", group_id),
                                    instance_path: format!("/vulnerabilities/{}/flags/{}/group_ids/{}", i_v, i_f, i_g),
                                });
                            }
                        }
                    }
                }
            }

            // Check vulnerability notes
            if let Some(notes) = vuln.get_notes() {
                for (i_n, note) in notes.iter().enumerate() {
                    if let Some(group_ids) = note.get_group_ids() {
                        for (i_g, group_id) in group_ids.enumerate() {
                            if !known_groups.contains(group_id) {
                                return Err(ValidationError {
                                    message: format!("Missing definition of product_group_id: {}", group_id),
                                    instance_path: format!("/vulnerabilities/{}/notes/{}/group_ids/{}", i_v, i_n, i_g),
                                });
                            }
                        }
                    }
                }
            }

            // Check vulnerability remediations
            for (i_r, remediation) in vuln.get_remediations().iter().enumerate() {
                if let Some(group_ids) = remediation.get_group_ids() {
                    for (i_g, group_id) in group_ids.collect::<Vec<_>>().iter().enumerate() {
                        if !known_groups.contains(*group_id) {
                            return Err(ValidationError {
                                message: format!("Missing definition of product_group_id: {}", group_id),
                                instance_path: format!("/vulnerabilities/{}/remediations/{}/group_ids/{}", i_v, i_r, i_g),
                            });
                        }
                    }
                }
            }

            // Check vulnerability threats
            for (i_t, threat) in vuln.get_threats().iter().enumerate() {
                if let Some(group_ids) = threat.get_group_ids() {
                    for (i_g, group_id) in group_ids.collect::<Vec<_>>().iter().enumerate() {
                        if !known_groups.contains(*group_id) {
                            return Err(ValidationError {
                                message: format!("Missing definition of product_group_id: {}", group_id),
                                instance_path: format!("/vulnerabilities/{}/threats/{}/group_ids/{}", i_v, i_t, i_g),
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
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_04::test_6_1_04_missing_definition_of_product_group_id;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_04() {
        let error01 = ValidationError {
            message: "Missing definition of product_group_id: CSAFGID-1020301".to_string(),
            instance_path: "/vulnerabilities/0/threats/0/group_ids/0".to_string(),
        };
        let error02 = ValidationError {
            message: "Missing definition of product_group_id: CSAFGID-1020300".to_string(),
            instance_path: "/vulnerabilities/0/flags/0/group_ids/0".to_string(),
        };
        let errors = HashMap::from([
            ("01", &error01),
            ("02", &error02),
        ]);
        run_csaf20_tests("04", test_6_1_04_missing_definition_of_product_group_id, &errors);
        run_csaf21_tests("04", test_6_1_04_missing_definition_of_product_group_id, &errors);
    }
}
use std::ops::Deref;
use crate::csaf::getter_traits::{ContentTrait, CsafTrait, DocumentTrait, MetricTrait, TrackingTrait, VulnerabilityIdTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;

pub fn test_6_1_47_inconsistent_ssvc_id(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let vulnerabilities = doc.get_vulnerabilities();

    for (i_v, v) in vulnerabilities.iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                return match m.get_content().get_ssvc_v1() {
                    Ok(ssvc) => {
                        // Get the SSVC ID
                        let ssvc_id = ssvc.id.deref();

                        // Check if SSVC ID equals document ID
                        let document_id = doc.get_document().get_tracking().get_id();
                        if ssvc_id == document_id {
                            // If there are multiple vulnerabilities, the validation must fail here.
                            if vulnerabilities.len() > 1 {
                                return Err(ValidationError {
                                    message: format!("The SSVC ID equals the document ID '{}' and the document contains multiple vulnerabilities", document_id),
                                    instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v1/id", i_v, i_m),
                                });
                            }
                            // SSVC ID is valid, go to next metrics object
                            continue;
                        }

                        // Check if it matches CVE
                        if let Some(cve) = v.get_cve() {
                            if ssvc_id == cve {
                                // SSVC ID is valid, go to next metrics object
                                continue;
                            }
                        }

                        // Check if it matches any ID in ids array
                        if let Some(ids) = v.get_ids() {
                            if ids.iter().any(|id| id.get_text() == ssvc_id) {
                                // SSVC ID is valid, go to next metrics object
                                continue;
                            }
                        }

                        // Return error if SSVC ID is not valid
                        Err(ValidationError {
                            message: format!("The SSVC ID '{}' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability", ssvc_id),
                            instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v1/id", i_v, i_m),
                        })
                    },
                    Err(err) => Err(ValidationError {
                        message: format!("Invalid SSVC object: {}", err),
                        instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v1", i_v, i_m),
                    }),
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
    use crate::csaf::validations::test_6_1_47::test_6_1_47_inconsistent_ssvc_id;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_47() {
        let instance_path = "/vulnerabilities/0/metrics/0/content/ssvc_v1/id".to_string();

        run_csaf21_tests(
            "47",
            test_6_1_47_inconsistent_ssvc_id,
            HashMap::from([
                ("01", &ValidationError {
                    message: "The SSVC ID 'CVE-1900-0002' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("02", &ValidationError {
                    message: "The SSVC ID 'CVE-1900-0001' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("03", &ValidationError {
                    message: "The SSVC ID '2723' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("04", &ValidationError {
                    message: "The SSVC ID 'Bug#2723' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
            ])
        );
    }
}
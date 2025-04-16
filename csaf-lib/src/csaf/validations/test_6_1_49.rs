use crate::csaf::csaf2_1::schema::DocumentStatus;
use crate::csaf::getter_traits::{ContentTrait, CsafTrait, DocumentTrait, MetricTrait, RevisionTrait, TrackingTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use chrono::{DateTime, FixedOffset};

/// 6.1.49 Inconsistent SSVC Timestamp
///
/// For each vulnerability, it is tested that the SSVC `timestamp` is earlier or equal to the `date`
/// of the newest item of the `revision_history` if the document status is `final` or `interim`.
pub fn test_6_1_49_inconsistent_ssvc_timestamp(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let document = doc.get_document();
    let tracking = document.get_tracking();
    let status = tracking.get_status();

    // Check if document status is "final" or "interim"
    if status != DocumentStatus::Final && status != DocumentStatus::Interim {
        return Ok(());
    }

    // Parse the date of each revision and find the newest one
    let mut newest_revision_date: Option<DateTime<FixedOffset>> = None;
    for (i_r, revision) in tracking.get_revision_history().iter().enumerate() {
        let date_str = revision.get_date();
        match DateTime::parse_from_rfc3339(date_str) {
            Ok(date) => {
                newest_revision_date = match newest_revision_date {
                    None => Some(date),
                    Some(newest_date) => Some(newest_date.max(date))
                };
            }
            Err(_) => {
                return Err(ValidationError {
                    message: format!("Invalid date format in revision history: {}", date_str),
                    instance_path: format!("/document/tracking/revision_history/{}/date", i_r),
                });
            }
        }
    }

    let newest_revision_date = match newest_revision_date {
        Some(date) => date,
        // No entries in revision history
        None => return Err(ValidationError {
            message: "Revision history must not be empty for status final or interim".to_string(),
            instance_path: "/document/tracking/revision_history".to_string(),
        }),
    };

    // Check each vulnerability's SSVC timestamp
    for (i_v, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (i_m, metric) in metrics.iter().enumerate() {
                match metric.get_content().get_ssvc_v1() {
                    Ok(ssvc) => {
                        if ssvc.timestamp.fixed_offset() > newest_revision_date {
                            return Err(ValidationError {
                                message: format!(
                                    "SSVC timestamp ({}) for vulnerability at index {} is later than the newest revision date ({})",
                                    ssvc.timestamp.to_rfc3339(), i_v, newest_revision_date.to_rfc3339()
                                ),
                                instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v1/timestamp", i_v, i_m),
                            })
                        }
                    },
                    Err(err) => {
                        return Err(ValidationError {
                            message: format!("Invalid SSVC object: {}", err),
                            instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v1", i_v, i_m),
                        });
                    },
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
    use crate::csaf::validations::test_6_1_49::test_6_1_49_inconsistent_ssvc_timestamp;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_49() {
        let instance_path = "/vulnerabilities/0/metrics/0/content/ssvc_v1/timestamp".to_string();

        run_csaf21_tests(
            "49",
            test_6_1_49_inconsistent_ssvc_timestamp,
            HashMap::from([
                ("01", &ValidationError {
                    message: "SSVC timestamp (2024-07-13T10:00:00+00:00) for vulnerability at index 0 is later than the newest revision date (2024-01-24T10:00:00+00:00)".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("02", &ValidationError {
                    message: "SSVC timestamp (2024-02-29T10:30:00+00:00) for vulnerability at index 0 is later than the newest revision date (2024-02-29T10:00:00+00:00)".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("03", &ValidationError {
                    message: "SSVC timestamp (2024-02-29T10:30:00+00:00) for vulnerability at index 0 is later than the newest revision date (2024-02-29T10:00:00+00:00)".to_string(),
                    instance_path: instance_path.clone(),
                }),
            ])
        );
    }
}
use crate::csaf::getter_traits::{CsafTrait, DocumentTrait, FlagTrait, GeneratorTrait, InvolvementTrait, RemediationTrait, RevisionTrait, ThreatTrait, TrackingTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use regex::Regex;
use std::sync::OnceLock;

static RFC3339_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_rfc3339_regex() -> &'static Regex {
    RFC3339_REGEX.get_or_init(||
        Regex::new(r"^((\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?)(Z|[+-]\d{2}:\d{2}))$").unwrap()
    )
}

/// Validates that all date/time fields in the CSAF document conform to the required format
/// (ISO 8601 format with time zone or UTC).
///
/// This function checks all date/time fields in the document, including tracking dates,
/// vulnerability disclosure/discovery dates, remediation dates, threat dates, etc.
pub fn test_6_1_37_date_and_time(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    let tracking = doc.get_document().get_tracking();

    // Check initial release date
    check_datetime(tracking.get_initial_release_date(), "/document/tracking/initial_release_date")?;

    // Check current release date
    check_datetime(tracking.get_current_release_date(), "/document/tracking/current_release_date")?;

    // Check generator date if present
    if let Some(generator) = tracking.get_generator() {
        if let Some(date) = generator.get_date() {
            check_datetime(date, "/document/tracking/generator/date")?;
        }
    }

    // Check revision history dates if present
    for (i_r, revision) in tracking.get_revision_history().iter().enumerate() {
        check_datetime(
            revision.get_date(),
            &format!("/document/tracking/revision_history/{}/date", i_r)
        )?;
    }

    // Check vulnerability related dates
    for (i_v, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // Check disclosure date if present
        if let Some(date) = vuln.get_release_date() {
            check_datetime(date, &format!("/vulnerabilities/{}/release_date", i_v))?;
        }

        // Check discovery date if present
        if let Some(date) = vuln.get_discovery_date() {
            check_datetime(date, &format!("/vulnerabilities/{}/discovery_date", i_v))?;
        }

        // Check flags dates if present
        if let Some(flags) = vuln.get_flags() {
            for (i_f, flag) in flags.iter().enumerate() {
                if let Some(date) = flag.get_date() {
                    check_datetime(date, &format!("/vulnerabilities/{}/flags/{}/date", i_v, i_f))?;
                }
            }
        }

        // Check involvements dates if present
        if let Some(involvements) = vuln.get_involvements() {
            for (i_i, involvement) in involvements.iter().enumerate() {
                if let Some(date) = involvement.get_date() {
                    check_datetime(
                        date,
                        &format!("/vulnerabilities/{}/involvements/{}/date", i_v, i_i)
                    )?;
                }
            }
        }

        // Check remediations dates if present
        for (i_r, remediation) in vuln.get_remediations().iter().enumerate() {
            if let Some(date) = remediation.get_date() {
                check_datetime(date, &format!("/vulnerabilities/{}/remediations/{}/date", i_v, i_r))?;
            }
        }

        // Check threats dates if present
        for (i_t, threat) in vuln.get_threats().iter().enumerate() {
            if let Some(date) = threat.get_date() {
                check_datetime(date, &format!("/vulnerabilities/{}/threats/{}/date", i_v, i_t))?;
            }
        }
    }

    Ok(())
}

fn check_datetime(date_time: &String, instance_path: &str) -> Result<(), ValidationError> {
    if get_rfc3339_regex().is_match(date_time) {
        // Add chrono-based plausibility check
        match chrono::DateTime::parse_from_rfc3339(date_time) {
            Ok(_) => Ok(()), // Successfully parsed as a valid RFC3339 datetime
            Err(e) => Err(ValidationError {
                message: format!("Date-time string {} matched RFC3339 regex but failed chrono parsing: {}", date_time, e),
                instance_path: instance_path.to_string(),
            }),
        }
    } else {
        Err(ValidationError {
            message: format!("Invalid date-time string {}, expected RFC3339-compliant format with non-empty timezone", date_time),
            instance_path: instance_path.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_37::test_6_1_37_date_and_time;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_37() {
        run_csaf21_tests(
            "37",
            test_6_1_37_date_and_time, HashMap::from([
                ("01", &ValidationError {
                    message: "Invalid date-time string 2024-01-24 10:00:00.000Z, expected RFC3339-compliant format with non-empty timezone".to_string(),
                    instance_path: "/document/tracking/initial_release_date".to_string(),
                }),
                ("02", &ValidationError {
                    message: "Invalid date-time string 2024-01-24T10:00:00.000z, expected RFC3339-compliant format with non-empty timezone".to_string(),
                    instance_path: "/document/tracking/initial_release_date".to_string(),
                }),
                ("03", &ValidationError {
                    message: "Date-time string 2014-13-31T00:00:00+01:00 matched RFC3339 regex but failed chrono parsing: input is out of range".to_string(),
                    instance_path: "/vulnerabilities/0/discovery_date".to_string(),
                }),
                ("04", &ValidationError {
                    message: "Date-time string 2023-02-30T00:00:00+01:00 matched RFC3339 regex but failed chrono parsing: input is out of range".to_string(),
                    instance_path: "/vulnerabilities/0/discovery_date".to_string(),
                }),
                ("05", &ValidationError {
                    message: "Date-time string 1900-02-29T00:00:00+01:00 matched RFC3339 regex but failed chrono parsing: input is out of range".to_string(),
                    instance_path: "/vulnerabilities/0/discovery_date".to_string(),
                }),
            ])
        );
    }
}

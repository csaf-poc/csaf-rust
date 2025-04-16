use crate::csaf::getter_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;

pub fn test_6_1_46_invalid_ssvc(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    // /vulnerabilities[]/metrics[]/content/ssvc_v1
    for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                m.get_content().get_ssvc_v1().map_err(|e| {
                    ValidationError {
                        message: format!("Invalid SSVC object: {}", e),
                        instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v1", i_v, i_m),
                    }
                })?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_46::test_6_1_46_invalid_ssvc;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_46() {
        run_csaf21_tests(
            "46",
            test_6_1_46_invalid_ssvc, HashMap::from([
                ("01", &ValidationError {
                    message: "Invalid SSVC object: missing field `selections`".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v1".to_string(),
                }),
                ("02", &ValidationError {
                    message: "Invalid SSVC object: unknown field `value`, expected one of `name`, `namespace`, `values`, `version`".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/0/content/ssvc_v1".to_string(),
                }),
            ])
        );
    }
}

use crate::csaf::getter_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use crate::csaf::validations::test_6_1_07::VulnerabilityMetrics::{
    CvssV2, CvssV30, CvssV31, CvssV4, Epss, SsvcV1,
};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

/// Types of metrics known until CSAF 2.1
#[derive(Hash, Eq, PartialEq, Clone)]
enum VulnerabilityMetrics {
    SsvcV1,
    CvssV2,
    CvssV30,
    CvssV31,
    CvssV4,
    Epss,
}

/// Display implementation for VulnerabilityMetrics.
impl Display for VulnerabilityMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SsvcV1 => write!(f, "SSVC-v1"),
            CvssV2 => write!(f, "CVSS-v2"),
            CvssV30 => write!(f, "CVSS-v3.0"),
            CvssV31 => write!(f, "CVSS-v3.1"),
            CvssV4 => write!(f, "CVSS-v4"),
            Epss => write!(f, "EPSS"),
        }
    }
}

/// Returns the name of the metric property for the given metric type.
fn get_metric_prop_name(metric: VulnerabilityMetrics) -> &'static str {
    match metric {
        SsvcV1 => "ssvc_v1",
        CvssV2 => "cvss_v2",
        CvssV30 => "cvss_v3",
        CvssV31 => "cvss_v3",
        CvssV4 => "cvss_v4",
        Epss => "epss",
    }
}

/// Test 6.1.7: Check for multiple identical metric types (with an identical source) per
/// vulnerability.
pub fn test_6_1_07_multiple_same_scores_per_product(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        let mut seen_metrics: HashMap<String, HashSet<(VulnerabilityMetrics, &Option<String>)>> =
            HashMap::new();
        if let Some(metrics) = v.get_metrics() {
            for (m_i, m) in metrics.iter().enumerate() {
                let content = m.get_content();
                let mut content_metrics = Vec::<(VulnerabilityMetrics, &Option<String>)>::new();
                if content.has_ssvc_v1() {
                    content_metrics.push((SsvcV1, m.get_source()));
                }
                if content.get_cvss_v2().is_some() {
                    content_metrics.push((CvssV2, m.get_source()));
                }
                if let Some(cvss_v3) = content.get_cvss_v3() {
                    if let Some(version) = cvss_v3.get("version") {
                        if version == "3.1" {
                            content_metrics.push((CvssV31, m.get_source()));
                        } else if version == "3.0" {
                            content_metrics.push((CvssV30, m.get_source()));
                        } else {
                            return Err(ValidationError {
                                message: format!("CVSS-v3 version {} is not supported.", version),
                                instance_path: format!(
                                    "{}/{}",
                                    content.get_content_json_path(v_i, m_i),
                                    get_metric_prop_name(CvssV30),
                                ),
                            });
                        }
                    }
                }
                if content.get_cvss_v4().is_some() {
                    content_metrics.push((CvssV4, m.get_source()));
                }
                if content.get_epss().is_some() {
                    content_metrics.push((Epss, m.get_source()));
                }
                for p in m.get_products() {
                    let metrics_set = seen_metrics.entry(p.to_string()).or_default();
                    for cm_src in content_metrics.iter() {
                        if metrics_set.contains(cm_src) {
                            return Err(ValidationError {
                                message: format!(
                                    "Product {} already has another metric \"{}\" {} assigned.",
                                    p,
                                    cm_src.0,
                                    match cm_src.1 {
                                        Some(src) => format!("with the same source \"{}\"", src),
                                        None => "without a source".to_string(),
                                    }
                                ),
                                instance_path: format!(
                                    "{}/{}",
                                    content.get_content_json_path(v_i, m_i),
                                    get_metric_prop_name(cm_src.0.to_owned())
                                ),
                            });
                        } else {
                            metrics_set.insert(cm_src.to_owned());
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
    use crate::csaf::validations::test_6_1_07::test_6_1_07_multiple_same_scores_per_product;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_07() {
        let cvss_v31_error_message = "Product CSAFPID-9080700 already has another metric \"CVSS-v3.1\" without a source assigned.";
        let csaf_20_path_prefix = "/vulnerabilities/0/scores/1";
        let csaf_21_path_prefix = "/vulnerabilities/0/metrics/1/content";
        run_csaf20_tests(
            "07",
            test_6_1_07_multiple_same_scores_per_product,
            &HashMap::from([(
                "01",
                &ValidationError {
                    message: cvss_v31_error_message.to_string(),
                    instance_path: format!("{}/cvss_v3", csaf_20_path_prefix),
                },
            )]),
        );
        run_csaf21_tests(
            "07",
            test_6_1_07_multiple_same_scores_per_product,
            &HashMap::from([
                ("01", &ValidationError {
                    message: cvss_v31_error_message.to_string(),
                    instance_path: format!("{}/cvss_v3", csaf_21_path_prefix),
                }),
                ("02", &ValidationError {
                    message: "Product CSAFPID-9080700 already has another metric \"CVSS-v3.0\" without a source assigned.".to_string(),
                    instance_path: format!("{}/cvss_v3", csaf_21_path_prefix),
                }),
                ("03", &ValidationError {
                    message: "Product CSAFPID-9080700 already has another metric \"CVSS-v2\" without a source assigned.".to_string(),
                    instance_path: format!("{}/cvss_v2", csaf_21_path_prefix),
                }),
                ("04", &ValidationError {
                    message: "Product CSAFPID-9080700 already has another metric \"CVSS-v4\" without a source assigned.".to_string(),
                    instance_path: format!("{}/cvss_v4", csaf_21_path_prefix),
                }),
                ("05", &ValidationError {
                    message: "Product CSAFPID-9080700 already has another metric \"CVSS-v3.1\" with the same source \
                    \"https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json\" assigned.".to_string(),
                    instance_path: format!("{}/cvss_v3", csaf_21_path_prefix),
                }),
            ]),
        );
    }
}

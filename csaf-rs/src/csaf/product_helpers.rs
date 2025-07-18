use crate::csaf::getter_traits::{CsafTrait, FlagTrait, MetricTrait, ProductGroupTrait, ProductStatusTrait, ProductTreeTrait, RelationshipTrait, RemediationTrait, ThreatTrait, VulnerabilityTrait};

pub fn gather_product_references(doc: &impl CsafTrait) -> Vec<(String, String)> {
    let mut ids = Vec::<(String, String)>::new();

    if let Some(pt) = doc.get_product_tree().as_ref() {
        // /product_tree/product_groups[]/product_ids[]
        for (g_i, g) in pt.get_product_groups().iter().enumerate() {
            for (i_i, i) in g.get_product_ids().enumerate() {
                ids.push(((*i).to_owned(), format!("/product_tree/product_groups/{}/product_ids/{}", g_i, i_i)))
            }
        }
        // /product_tree/relationships[]/product_reference
        // /product_tree/relationships[]/relates_to_product_reference
        for (r_i, r) in pt.get_relationships().iter().enumerate() {
            ids.push((r.get_product_reference().to_owned(), format!("/product_tree/relationships/{}/product_reference", r_i)));
            ids.push((r.get_relates_to_product_reference().to_owned(), format!("/product_tree/relationships/{}/relates_to_product_reference", r_i)));
        }
    }

    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        // /vulnerabilities[]/product_status/first_affected[]
        // /vulnerabilities[]/product_status/first_fixed[]
        // /vulnerabilities[]/product_status/fixed[]
        // /vulnerabilities[]/product_status/known_affected[]
        // /vulnerabilities[]/product_status/known_not_affected[]
        // /vulnerabilities[]/product_status/last_affected[]
        // /vulnerabilities[]/product_status/recommended[]
        // /vulnerabilities[]/product_status/under_investigation[]
        if let Some(status) = v.get_product_status().as_ref() {
            if let Some(fa) = status.get_first_affected() {
                for (x_i, x) in fa.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/first_affected/{}", v_i, x_i)));
                }
            }
            if let Some(ff) = status.get_first_fixed() {
                for (x_i, x) in ff.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/first_fixed/{}", v_i, x_i)));
                }
            }
            if let Some(f) = status.get_fixed() {
                for (x_i, x) in f.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/fixed/{}", v_i, x_i)));
                }
            }
            if let Some(ka) = status.get_known_affected() {
                for (x_i, x) in ka.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/known_affected/{}", v_i, x_i)));
                }
            }
            if let Some(kna) = status.get_known_not_affected() {
                for (x_i, x) in kna.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/known_not_affected/{}", v_i, x_i)));
                }
            }
            if let Some(la) = status.get_last_affected() {
                for (x_i, x) in la.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/last_affected/{}", v_i, x_i)));
                }
            }
            if let Some(r) = status.get_recommended() {
                for (x_i, x) in r.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/recommended/{}", v_i, x_i)));
                }
            }
            if let Some(ui) = status.get_under_investigation() {
                for (x_i, x) in ui.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/product_status/under_investigation/{}", v_i, x_i)));
                }
            }
        }

        // /vulnerabilities[]/remediations[]/product_ids[]
        for (rem_i, rem) in v.get_remediations().iter().enumerate() {
            if let Some(product_ids) = rem.get_product_ids() {
                for (x_i, x) in product_ids.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/remediations/{}/product_ids/{}", v_i, rem_i, x_i)));
                }
            }
        }

        // /vulnerabilities[]/metrics[]/products[]
        if let Some(metrics) = v.get_metrics().as_ref() {
            for (metric_i, metric) in metrics.iter().enumerate() {
                for (x_i, x) in metric.get_products().enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/metrics/{}/products/{}", v_i, metric_i, x_i)));
                }
            }
        }

        // /vulnerabilities[]/threats[]/product_ids[]
        for (threat_i, threat) in v.get_threats().iter().enumerate() {
            if let Some(product_ids) = threat.get_product_ids() {
                for (x_i, x) in product_ids.enumerate() {
                    ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/threats/{}/product_ids/{}", v_i, threat_i, x_i)));
                }
            }
        }

        // /vulnerabilities[]/flags[]/product_ids[]
        if let Some(flags) = v.get_flags().as_ref() {
            for (flag_i, flag) in flags.iter().enumerate() {
                if let Some(product_ids) = flag.get_product_ids() {
                    for (x_i, x) in product_ids.enumerate() {
                        ids.push(((*x).to_owned(), format!("/vulnerabilities/{}/flags/{}/product_ids/{}", v_i, flag_i, x_i)));
                    }
                }
            }
        }
    }

    ids
}

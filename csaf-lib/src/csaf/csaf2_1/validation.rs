use super::schema::CommonSecurityAdvisoryFramework;
use crate::csaf::validation::{Test, Validatable, ValidationPreset};
use crate::csaf::validations::test_6_1_01::test_6_1_01_missing_definition_of_product_id;
use crate::csaf::validations::test_6_1_02::test_6_1_02_multiple_definition_of_product_id;
use crate::csaf::validations::test_6_1_34::test_6_1_34_branches_recursion_depth;
use crate::csaf::validations::test_6_1_35::test_6_1_35_contradicting_remediations;
use std::collections::HashMap;

impl Validatable<CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>> {
        let basic_tests = Vec::from(["6.1.1", "6.1.2", "6.1.34", "6.1.35"]);
        // More tests may be added in extend() here later
        let extended_tests: Vec<&str> = basic_tests.clone();
        // extended_tests.extend(["foo"].iter());
        let full_tests: Vec<&str> = extended_tests.clone();
        // full_tests.extend(["bar"].iter());
        HashMap::from([
            (ValidationPreset::Basic, basic_tests),
            (ValidationPreset::Extended, extended_tests),
            (ValidationPreset::Full, full_tests),
        ])
    }

    fn tests(&self) -> HashMap<&str, Test<CommonSecurityAdvisoryFramework>> {
        type CsafTest = Test<CommonSecurityAdvisoryFramework>;
        HashMap::from([
            ("6.1.1", test_6_1_01_missing_definition_of_product_id as CsafTest),
            ("6.1.2", test_6_1_02_multiple_definition_of_product_id as CsafTest),
            ("6.1.34", test_6_1_34_branches_recursion_depth as CsafTest),
            ("6.1.35", test_6_1_35_contradicting_remediations as CsafTest),
        ])
    }

    fn doc(&self) -> &CommonSecurityAdvisoryFramework {
        self
    }
}

use super::schema::{CommonSecurityAdvisoryFramework};
use crate::csaf::validation::{Test, Validatable, ValidationPreset};
use crate::csaf::validations::test_6_1_01::test_6_1_01_missing_definition_of_product_id;
use crate::csaf::validations::test_6_1_02::test_6_1_02_multiple_definition_of_product_id;
use crate::csaf::validations::test_6_1_34::test_6_1_34_branches_recursion_depth;
use crate::csaf::validations::test_6_1_35::test_6_1_35_contradicting_remediations;
use crate::csaf::validations::test_6_1_36::test_6_1_36_status_group_contradicting_remediation_categories;
use crate::csaf::validations::test_6_1_37::test_6_1_37_date_and_time;
use crate::csaf::validations::test_6_1_38::test_6_1_38_non_public_sharing_group_max_uuid;
use crate::csaf::validations::test_6_1_39::test_6_1_39_public_sharing_group_with_no_max_uuid;
use crate::csaf::validations::test_6_1_40::test_6_1_40_invalid_sharing_group_name;
use crate::csaf::validations::test_6_1_41::test_6_1_41_missing_sharing_group_name;
use crate::csaf::validations::test_6_1_42::test_6_1_42_purl_consistency;
use std::collections::HashMap;

impl Validatable<CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>> {
        let basic_tests = Vec::from([
            "6.1.1", "6.1.2", "6.1.34", "6.1.35", "6.1.36", "6.1.37",
            "6.1.38", "6.1.39", "6.1.40", "6.1.41", "6.1.42"
        ]);
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
            ("6.1.36", test_6_1_36_status_group_contradicting_remediation_categories as CsafTest),
            ("6.1.37", test_6_1_37_date_and_time as CsafTest),
            ("6.1.38", test_6_1_38_non_public_sharing_group_max_uuid as CsafTest),
            ("6.1.39", test_6_1_39_public_sharing_group_with_no_max_uuid as CsafTest),
            ("6.1.40", test_6_1_40_invalid_sharing_group_name as CsafTest),
            ("6.1.41", test_6_1_41_missing_sharing_group_name as CsafTest),
            ("6.1.42", test_6_1_42_purl_consistency as CsafTest),
        ])
    }

    fn doc(&self) -> &CommonSecurityAdvisoryFramework {
        self
    }
}

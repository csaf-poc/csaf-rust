use crate::csaf::csaf2_0::schema::{Branch, CategoryOfTheRemediation, CommonSecurityAdvisoryFramework, DocumentGenerator, DocumentLevelMetaData, DocumentStatus, Flag, FullProductNameT, HelperToIdentifyTheProduct, Id, Involvement, LabelOfTlp, Note, ProductGroup, ProductStatus, ProductTree, Relationship, Remediation, Revision, RulesForSharingDocument, Score, Threat, Tracking, TrafficLightProtocolTlp, Vulnerability};
use crate::csaf::csaf2_1::schema::{CategoryOfTheRemediation as Remediation21, DocumentStatus as Status21, Epss, LabelOfTlp as Tlp21};
use crate::csaf::getter_traits::{BranchTrait, CsafTrait, DistributionTrait, DocumentTrait, FlagTrait, ProductTrait, GeneratorTrait, InvolvementTrait, MetricTrait, ProductGroupTrait, ProductIdentificationHelperTrait, ProductStatusTrait, ProductTreeTrait, RelationshipTrait, RemediationTrait, RevisionTrait, SharingGroupTrait, ThreatTrait, TlpTrait, TrackingTrait, VulnerabilityTrait, ContentTrait, VulnerabilityIdTrait, NoteTrait, WithGroupIds, FirstKnownExploitationDatesTrait};
use std::ops::Deref;
use serde::de::Error;
use serde_json::{Map, Value};
use uuid::Uuid;
use crate::csaf::csaf2_1::ssvc_schema::SsvcV1;
use crate::csaf::validation::ValidationError;

impl WithGroupIds for Remediation {
    fn get_group_ids(&self) -> Option<impl Iterator<Item=&String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl RemediationTrait for Remediation {
    /// Normalizes the remediation categories from CSAF 2.0 to those of CSAF 2.1.
    ///
    /// # Explanation
    /// In CSAF 2.1, the list of remediation categories was expanded, making it a superset of those
    /// in CSAF 2.0. This function ensures that the remediation category from a CSAF 2.0 remediation
    /// object is converted into the corresponding category defined in CSAF 2.1.
    ///
    /// # Returns
    /// A CSAF 2.1 `CategoryOfTheRemediation` that corresponds to the remediation category of the
    /// current object.
    fn get_category(&self) -> Remediation21 {
        match self.category {
            CategoryOfTheRemediation::Workaround => Remediation21::Workaround,
            CategoryOfTheRemediation::Mitigation => Remediation21::Mitigation,
            CategoryOfTheRemediation::VendorFix => Remediation21::VendorFix,
            CategoryOfTheRemediation::NoFixPlanned => Remediation21::NoFixPlanned,
            CategoryOfTheRemediation::NoneAvailable => Remediation21::NoneAvailable,
        }
    }

    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_date(&self) -> &Option<String> {
        &self.date
    }
}

impl ProductStatusTrait for ProductStatus {
    fn get_first_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_first_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.first_fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_fixed(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.fixed.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_known_not_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.known_not_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_last_affected(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.last_affected.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_recommended(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.recommended.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_under_investigation(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.under_investigation.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    /// Not specified for CSAF 2.0, so `None`
    fn get_unknown(&self) -> Option<impl Iterator<Item=&String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl MetricTrait for Score {
    type ContentType = Score;

    fn get_products(&self) -> impl Iterator<Item = &String> + '_ {
        self.products.iter().map(|x| x.deref())
    }

    fn get_content(&self) -> &Self::ContentType {
        self
    }

    fn get_source(&self) -> &Option<String> {
        &None
    }
}

impl ContentTrait for Score {
    fn has_ssvc_v1(&self) -> bool {
        false
    }

    fn get_ssvc_v1(&self) -> Result<SsvcV1, serde_json::Error> {
        Err(serde_json::Error::custom("SSVC metrics are not implemented in CSAF 2.0"))
    }

    fn get_cvss_v2(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v2.is_empty() {
            None
        } else {
            Some(&self.cvss_v2)
        }
    }

    fn get_cvss_v3(&self) -> Option<&Map<String, Value>> {
        if self.cvss_v3.is_empty() {
            None
        } else {
            Some(&self.cvss_v3)
        }
    }

    fn get_cvss_v4(&self) -> Option<&Map<String, Value>> {
        None
    }

    fn get_epss(&self) -> &Option<Epss> {
        &None::<Epss>
    }

    fn get_content_json_path(&self, vulnerability_idx: usize, metric_idx: usize) -> String {
        format!(
            "/vulnerabilities/{}/scores/{}",
            vulnerability_idx, metric_idx
        )
    }
}

impl WithGroupIds for Threat {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl ThreatTrait for Threat {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }

    fn get_date(&self) -> &Option<String> {
        &self.date
    }
}

impl VulnerabilityTrait for Vulnerability {
    type RemediationType = Remediation;
    type ProductStatusType = ProductStatus;
    // Metrics are not implemented in CSAF 2.0
    type MetricType = Score;
    type ThreatType = Threat;
    type FlagType = Flag;
    type InvolvementType = Involvement;
    type VulnerabilityIdType = Id;
    type NoteType = Note;
    // First known exploitation dates are not implemented in CSAF 2.0
    type FirstKnownExploitationDatesType = ();

    fn get_remediations(&self) -> &Vec<Self::RemediationType> {
        &self.remediations
    }

    fn get_product_status(&self) -> &Option<Self::ProductStatusType> {
        &self.product_status
    }

    fn get_metrics(&self) -> Option<&Vec<Self::MetricType>> {
        Some(&self.scores)
    }

    fn get_threats(&self) -> &Vec<Self::ThreatType> {
        &self.threats
    }

    fn get_disclosure_date(&self) -> &Option<String> {
        &self.release_date
    }

    fn get_discovery_date(&self) -> &Option<String> {
        &self.discovery_date
    }

    fn get_flags(&self) -> &Option<Vec<Self::FlagType>> {
        &self.flags
    }

    fn get_involvements(&self) -> &Option<Vec<Self::InvolvementType>> {
        &self.involvements
    }

    fn get_cve(&self) -> Option<&String> {
        self.cve.as_ref().map(|x| x.deref())
    }
    
    fn get_ids(&self) -> &Option<Vec<Self::VulnerabilityIdType>> {
        &self.ids
    }

    fn get_notes(&self) -> Option<&Vec<Self::NoteType>> {
        self.notes.as_ref().map(|x| x.deref())
    }

    fn get_first_known_exploitation_dates(&self) -> Option<&Vec<Self::FirstKnownExploitationDatesType>> {
        None
    }
}

impl VulnerabilityIdTrait for Id {
    fn get_system_name(&self) -> &String {
        self.system_name.deref()
    }

    fn get_text(&self) -> &String {
        self.text.deref()
    }
}

impl WithGroupIds for Flag {
    fn get_group_ids(&self) -> Option<impl Iterator<Item=&String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl FlagTrait for Flag {
    fn get_date(&self) -> &Option<String> {
        &self.date
    }

    fn get_product_ids(&self) -> Option<impl Iterator<Item=&String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl FirstKnownExploitationDatesTrait for () {
    fn get_date(&self) -> &String {
        panic!("First known exploitation dates are not implemented in CSAF 2.0");
    }
}

impl InvolvementTrait for Involvement {
    fn get_date(&self) -> &Option<String> {
        &self.date
    }
}

impl CsafTrait for CommonSecurityAdvisoryFramework {
    type VulnerabilityType = Vulnerability;
    type ProductTreeType = ProductTree;
    type DocumentType = DocumentLevelMetaData;

    fn get_product_tree(&self) -> &Option<Self::ProductTreeType> {
        &self.product_tree
    }

    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType> {
        &self.vulnerabilities
    }

    fn get_document(&self) -> &Self::DocumentType {
        &self.document
    }
}

impl DocumentTrait for DocumentLevelMetaData {
    type TrackingType = Tracking;
    type DistributionType = RulesForSharingDocument;
    type NoteType = Note;

    fn get_tracking(&self) -> &Self::TrackingType {
        &self.tracking
    }

    /// Return distribution as ref Option, it is optional anyways
    fn get_distribution_20(&self) -> Option<&Self::DistributionType> {
        self.distribution.as_ref()
    }

    /// Return distribution or a Validation error to satisfy CSAF 2.1 semantics
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, ValidationError> {
        match self.distribution.as_ref() {
            None => Err(ValidationError {
                message: "CSAF 2.1 requires the distribution property, but it is not set.".to_string(),
                instance_path: "/document/distribution".to_string()
            }),
            Some(distribution) => Ok(distribution)
        }
    }

    fn get_notes(&self) -> Option<&Vec<Self::NoteType>> {
        self.notes.as_ref().map(|x| x.deref())
    }

    fn get_lang(&self) -> Option<&String> {
        self.lang.as_ref().map(|x| x.deref())
    }

    fn get_source_lang(&self) -> Option<&String> {
        self.source_lang.as_ref().map(|x| x.deref())
    }
}

impl DistributionTrait for RulesForSharingDocument {
    type SharingGroupType = ();
    type TlpType = TrafficLightProtocolTlp;

    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType> {
        &None
    }

    /// Return TLP as ref Option, it is an option anyway
    fn get_tlp_20(&self) -> Option<&Self::TlpType> {
        self.tlp.as_ref()
    }

    /// Return TLP or a ValidationError to satisfy CSAF 2.1 semantics
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError> {
        match self.tlp.as_ref() {
            None => Err(ValidationError {
                message: "CSAF 2.1 requires the TLP property, but it is not set.".to_string(),
                instance_path: "/document/distribution/sharing_group/tlp".to_string()
            }),
            Some(tlp) => Ok(tlp)
        }
    }
}

impl WithGroupIds for Note {
    fn get_group_ids(&self) -> Option<impl Iterator<Item=&String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl NoteTrait for Note {}

impl SharingGroupTrait for () {
    fn get_id(&self) -> &Uuid {
        panic!("Sharing groups are not implemented in CSAF 2.0");
    }

    fn get_name(&self) -> Option<&String> {
        panic!("Sharing groups are not implemented in CSAF 2.0");
    }
}

impl TlpTrait for TrafficLightProtocolTlp {
    /// Normalizes the TLP (Traffic Light Protocol) labels from CSAF 2.0 to those of CSAF 2.1.
    ///
    /// # Explanation
    /// In CSAF 2.1, the TLP labeling scheme was updated to align with the official TLP 2.0 standard,
    /// which renamed "WHITE" to "CLEAR". This function ensures that TLP labels from CSAF 2.0
    /// are converted to their corresponding labels in CSAF 2.1.
    ///
    /// # Returns
    /// A CSAF 2.1 `Tlp21` value that corresponds to the TLP label of the current object.
    fn get_label(&self) -> Tlp21 {
        match self.label {
            LabelOfTlp::Amber => Tlp21::Amber,
            LabelOfTlp::Green => Tlp21::Green,
            LabelOfTlp::Red => Tlp21::Red,
            LabelOfTlp::White => Tlp21::Clear,
        }
    }
}

impl TrackingTrait for Tracking {
    type GeneratorType = DocumentGenerator;
    type RevisionType = Revision;

    fn get_current_release_date(&self) -> &String {
        &self.current_release_date
    }

    fn get_initial_release_date(&self) -> &String {
        &self.initial_release_date
    }

    fn get_generator(&self) -> &Option<Self::GeneratorType> {
        &self.generator
    }

    fn get_revision_history(&self) -> &Vec<Self::RevisionType> {
        &self.revision_history
    }

    fn get_status(&self) -> Status21 {
        match self.status {
            DocumentStatus::Draft => Status21::Draft,
            DocumentStatus::Final => Status21::Final,
            DocumentStatus::Interim => Status21::Interim,
        }
    }

    fn get_id(&self) -> &String {
        self.id.deref()
    }
}

impl GeneratorTrait for DocumentGenerator {
    fn get_date(&self) -> &Option<String> {
        &self.date
    }
}

impl RevisionTrait for Revision {
    fn get_date(&self) -> &String {
        &self.date
    }
    fn get_number(&self) -> &String {
        &self.number
    }
    fn get_summary(&self) -> &String {
        &self.summary
    }
}

impl ProductTreeTrait for ProductTree {
    type BranchType = Branch;
    type ProductGroupType = ProductGroup;
    type RelationshipType = Relationship;
    type FullProductNameType = FullProductNameT;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_ref().map(|branches| branches.deref())
    }

    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType> {
        &self.product_groups
    }

    fn get_relationships(&self) -> &Vec<Self::RelationshipType> {
        &self.relationships
    }

    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType> {
        &self.full_product_names
    }

    fn visit_all_products(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str) -> Result<(), ValidationError>) -> Result<(), ValidationError> {
        self.visit_all_products_generic(callback)
    }
}

impl BranchTrait<FullProductNameT> for Branch {
    fn get_branches(&self) -> Option<&Vec<Self>> {
        self.branches.as_ref().map(|branches| branches.deref())
    }

    fn get_product(&self) -> &Option<FullProductNameT> {
        &self.product
    }
}

impl ProductGroupTrait for ProductGroup {
    fn get_group_id(&self) -> &String {
        self.group_id.deref()
    }

    fn get_product_ids(&self) -> impl Iterator<Item = &String> + '_ {
        self.product_ids.iter().map(|x| x.deref())
    }
}

impl RelationshipTrait<FullProductNameT> for Relationship {
    fn get_product_reference(&self) -> &String {
        self.product_reference.deref()
    }

    fn get_relates_to_product_reference(&self) -> &String {
        self.relates_to_product_reference.deref()
    }

    fn get_full_product_name(&self) -> &FullProductNameT {
        &self.full_product_name
    }
}

impl ProductTrait for FullProductNameT {
    type ProductIdentificationHelperType = HelperToIdentifyTheProduct;

    fn get_product_id(&self) -> &String {
        self.product_id.deref()
    }

    fn get_product_identification_helper(&self) -> &Option<Self::ProductIdentificationHelperType> {
        &self.product_identification_helper
    }
}

impl ProductIdentificationHelperTrait for HelperToIdentifyTheProduct {
    fn get_purls(&self) -> Option<&[String]> {
        self.purl.as_ref().map(|purl| std::slice::from_ref(purl))
    }

    fn get_model_numbers(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.model_numbers.as_ref().map(|v| v.iter().map(|x| x.deref()))
    }

    fn get_serial_numbers(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.serial_numbers.as_ref().map(|v| v.iter().map(|x| x.deref()))
    }
}
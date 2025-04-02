use crate::csaf::csaf2_0::schema::{Branch, CategoryOfTheRemediation, CommonSecurityAdvisoryFramework, DocumentGenerator, DocumentLevelMetaData, DocumentStatus, Flag, FullProductNameT, HelperToIdentifyTheProduct, Involvement, LabelOfTlp, ProductGroup, ProductStatus, ProductTree, Relationship, Remediation, Revision, RulesForSharingDocument, Threat, Tracking, TrafficLightProtocolTlp, Vulnerability};
use crate::csaf::csaf2_1::schema::{CategoryOfTheRemediation as Remediation21, DocumentStatus as Status21, LabelOfTlp as Tlp21};
use crate::csaf::getter_traits::{BranchTrait, CsafTrait, DistributionTrait, DocumentTrait, FlagTrait, FullProductNameTrait, GeneratorTrait, InvolvementTrait, MetricTrait, ProductGroupTrait, ProductIdentificationHelperTrait, ProductStatusTrait, ProductTreeTrait, RelationshipTrait, RemediationTrait, RevisionTrait, SharingGroupTrait, ThreatTrait, TlpTrait, TrackingTrait, VulnerabilityTrait};
use std::ops::Deref;
use crate::csaf::validation::ValidationError;

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

    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
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
}

impl MetricTrait for () {
    //noinspection RsConstantConditionIf
    fn get_products(&self) -> impl Iterator<Item = &String> + '_ {
        // This construction is required to satisfy compiler checks
        // and still panic if this is ever called (as this would be a clear error!).
        if true {
            panic!("Metrics are not implemented in CSAF 2.0");
        }
        std::iter::empty()
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
    type MetricType = ();
    type ThreatType = Threat;
    type FlagType = Flag;
    type InvolvementType = Involvement;

    fn get_remediations(&self) -> &Vec<Self::RemediationType> {
        &self.remediations
    }

    fn get_product_status(&self) -> &Option<Self::ProductStatusType> {
        &self.product_status
    }

    fn get_metrics(&self) -> &Option<Vec<Self::MetricType>> {
        // Metrics are not implemented in CSAF 2.0
        &None
    }

    fn get_threats(&self) -> &Vec<Self::ThreatType> {
        &self.threats
    }

    fn get_release_date(&self) -> &Option<String> {
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
}

impl FlagTrait for Flag {
    fn get_date(&self) -> &Option<String> {
        &self.date
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

impl SharingGroupTrait for () {
    fn get_id(&self) -> &String {
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
}

impl BranchTrait for Branch {
    type BranchType = Branch;
    type FullProductNameType = FullProductNameT;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_ref().map(|branches| branches.deref())
    }

    fn get_product(&self) -> &Option<Self::FullProductNameType> {
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

impl RelationshipTrait for Relationship {
    type FullProductNameType = FullProductNameT;

    fn get_product_reference(&self) -> &String {
        self.product_reference.deref()
    }

    fn get_relates_to_product_reference(&self) -> &String {
        self.relates_to_product_reference.deref()
    }

    fn get_full_product_name(&self) -> &Self::FullProductNameType {
        &self.full_product_name
    }
}

impl FullProductNameTrait for FullProductNameT {
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
}
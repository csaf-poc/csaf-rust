use crate::csaf::csaf2_1::schema::{Branch, CategoryOfTheRemediation, CommonSecurityAdvisoryFramework, DocumentGenerator, DocumentLevelMetaData, DocumentStatus, Flag, FullProductNameT, HelperToIdentifyTheProduct, Involvement, LabelOfTlp, Metric, ProductGroup, ProductStatus, ProductTree, Relationship, Remediation, Revision, RulesForSharingDocument, SharingGroup, Threat, Tracking, TrafficLightProtocolTlp, Vulnerability};
use crate::csaf::getter_traits::{BranchTrait, CsafTrait, DistributionTrait, DocumentTrait, FlagTrait, FullProductNameTrait, GeneratorTrait, InvolvementTrait, MetricTrait, ProductGroupTrait, ProductIdentificationHelperTrait, ProductStatusTrait, ProductTreeTrait, RelationshipTrait, RemediationTrait, RevisionTrait, SharingGroupTrait, ThreatTrait, TlpTrait, TrackingTrait, VulnerabilityTrait};
use std::ops::Deref;
use crate::csaf::validation::ValidationError;

impl RemediationTrait for Remediation {
    fn get_category(&self) -> CategoryOfTheRemediation {
        self.category
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

impl MetricTrait for Metric {
    fn get_products(&self) -> impl Iterator<Item = &String> + '_ {
        self.products.deref().iter().map(|p| p.deref())
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
    type MetricType = Metric;
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
        &self.metrics
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

    /// We normalize to Option here because property was optional in CSAF 2.0
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, ValidationError> {
        Ok(&self.distribution)
    }

    /// Always return the value because it is mandatory
    fn get_distribution_20(&self) -> Option<&Self::DistributionType> {
        Some(&self.distribution)
    }
}

impl DistributionTrait for RulesForSharingDocument {
    type SharingGroupType = SharingGroup;
    type TlpType = TrafficLightProtocolTlp;

    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType> {
        &self.sharing_group
    }

    /// We normalize to Option here because property was optional in CSAF 2.0
    fn get_tlp_20(&self) -> Option<&Self::TlpType> {
        Some(&self.tlp)
    }

    /// Always return the value because it is mandatory
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError> {
        Ok(&self.tlp)
    }
}

impl SharingGroupTrait for SharingGroup {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_name(&self) -> Option<&String> {
        self.name.as_ref().map(|x| x.deref())
    }
}

impl TlpTrait for TrafficLightProtocolTlp {
    fn get_label(&self) -> LabelOfTlp {
        self.label
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

    fn get_status(&self) -> DocumentStatus {
        self.status
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
        self.purls.as_ref().map(|v| v.as_slice())
    }
}

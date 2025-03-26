use std::collections::{BTreeSet, HashSet};
use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::helpers::resolve_product_groups;

/// Trait representing an abstract Common Security Advisory Framework (CSAF) document.
///
/// The `CsafTrait` trait defines the key structure of a CSAF document, allowing
/// interaction with its vulnerabilities and product tree without tying to a
/// specific version of the CSAF schema.
pub trait CsafTrait {
    /// The associated type representing the type of vulnerabilities in this CSAF structure.
    type VulnerabilityType: VulnerabilityTrait;

    /// The associated type representing the type of product tree in this CSAF structure.
    type ProductTreeType: ProductTreeTrait;

    /// The associated type representing the type of document meta in this CSAF structure.
    type DocumentType: DocumentTrait;

    /// Returns the product tree of the CSAF document, if available.
    fn get_product_tree(&self) -> Option<Self::ProductTreeType>;

    /// Retrieves all vulnerabilities present in the CSAF document.
    fn get_vulnerabilities(&self) -> Vec<Self::VulnerabilityType>;

    /// Retrieves the document meta present in the CSAF document.
    fn get_document(&self) -> Self::DocumentType;
}

pub trait DocumentTrait {
    type TrackingType: TrackingTrait;

    fn get_tracking(&self) -> Self::TrackingType;
}

pub trait TrackingTrait {
    /// Type representing document generator information
    type GeneratorType: GeneratorTrait;

    /// Type representing revision history entries
    type RevisionType: RevisionTrait;

    /// The release date of the latest version of this document
    fn get_current_release_date(&self) -> &String;

    /// The initial release date of this document
    fn get_initial_release_date(&self) -> &String;

    /// Returns the generator information for this document
    fn get_generator(&self) -> &Option<Self::GeneratorType>;

    /// Returns the revision history for this document
    fn get_revision_history(&self) -> &Vec<Self::RevisionType>;
}

/// Trait for accessing document generator information
pub trait GeneratorTrait {
    /// Returns the date when this document was generated
    fn get_date(&self) -> &Option<String>;
}

/// Trait for accessing revision history entry information
pub trait RevisionTrait {
    /// Returns the date associated with this revision entry
    fn get_date(&self) -> &String;

    /// Returns the number/identifier of this revision
    fn get_number(&self) -> &String;

    /// Returns the summary of changes in this revision
    fn get_summary(&self) -> &String;
}


/// Trait representing an abstract vulnerability in a CSAF document.
///
/// The `VulnerabilityTrait` defines the structure of a vulnerability and includes
/// information about potential remediations.
pub trait VulnerabilityTrait {
    /// The associated type representing the type of remediations in a vulnerability.
    type RemediationType: RemediationTrait;

    /// The associated type representing the product status information.
    type ProductStatusType: ProductStatusTrait;

    /// The associated type representing the metric information.
    type MetricType: MetricTrait;

    /// The associated type representing the threat information.
    type ThreatType: ThreatTrait;

    /// The type representing a vulnerability flag
    type FlagType: FlagTrait;

    /// The type representing a vulnerability involvement
    type InvolvementType: InvolvementTrait;

    /// Retrieves a list of remediations associated with the vulnerability.
    fn get_remediations(&self) -> Vec<Self::RemediationType>;

    /// Retrieves the status of products affected by the vulnerability, if available.
    fn get_product_status(&self) -> Option<Self::ProductStatusType>;

    /// Returns an optional vector of metrics related to the vulnerability.
    fn get_metrics(&self) -> Option<Vec<Self::MetricType>>;

    /// Retrieves a list of potential threats related to the vulnerability.
    fn get_threats(&self) -> Vec<Self::ThreatType>;

    /// Returns the date when this vulnerability was initially disclosed
    fn get_release_date(&self) -> &Option<String>;

    /// Returns the date when this vulnerability was initially discovered
    fn get_discovery_date(&self) -> &Option<String>;

    /// Returns all flags associated with this vulnerability
    fn get_flags(&self) -> &Option<Vec<Self::FlagType>>;

    /// Returns all involvements associated with this vulnerability
    fn get_involvements(&self) -> &Option<Vec<Self::InvolvementType>>;
}

/// Trait for accessing vulnerability flags information
pub trait FlagTrait {
    /// Returns the date associated with this vulnerability flag
    fn get_date(&self) -> &Option<String>;
}

/// Trait for accessing vulnerability involvement information
pub trait InvolvementTrait {
    /// Returns the date associated with this vulnerability involvement
    fn get_date(&self) -> &Option<String>;
}

/// Trait representing an abstract remediation in a CSAF document.
///
/// The `RemediationTrait` encapsulates the details of a remediation, such as its
/// category and the affected products or groups.
pub trait RemediationTrait {
    /// Returns the category of the remediation.
    ///
    /// Categories are defined by the CSAF schema.
    fn get_category(&self) -> CategoryOfTheRemediation;

    /// Retrieves the product IDs directly affected by this remediation, if any.
    fn get_product_ids(&self) -> Option<Vec<&String>>;

    /// Retrieves the product group IDs related to this remediation, if any.
    fn get_group_ids(&self) -> Option<Vec<&String>>;

    /// Computes a set of all product IDs affected by this remediation, either
    /// directly or through product groups.
    ///
    /// # Arguments
    ///
    /// * `doc` - A reference to the CSAF document to resolve product groups.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<String>` containing all product IDs, or `None` if none exist.
    fn get_all_product_ids(&self, doc: &impl CsafTrait) -> Option<BTreeSet<String>> {
        if self.get_product_ids().is_none() && self.get_group_ids().is_none() {
            None
        } else {
            let mut product_set: BTreeSet<String> = match self.get_product_ids() {
                Some(product_ids) => product_ids.iter().map(|id| (*id).to_owned()).collect(),
                None => BTreeSet::new(),
            };
            if let Some(product_groups) = self.get_group_ids() {
                if let Some(product_ids) = resolve_product_groups(doc, product_groups) {
                    product_set.extend(product_ids.iter().map(|id| id.to_owned()));
                }
            }
            Some(product_set)
        }
    }

    /// Returns the date associated with this remediation
    fn get_date(&self) -> &Option<String>;
}

/// Trait representing an abstract product status in a CSAF document.
pub trait ProductStatusTrait {
    /// Returns a reference to the list of first affected product IDs.
    fn get_first_affected(&self) -> Option<Vec<&String>>;

    /// Returns a reference to the list of first fixed product IDs.
    fn get_first_fixed(&self) -> Option<Vec<&String>>;

    /// Returns a reference to the list of fixed product IDs.
    fn get_fixed(&self) -> Option<Vec<&String>>;

    /// Returns a reference to the list of known affected product IDs.
    fn get_known_affected(&self) -> Option<Vec<&String>>;

    /// Returns a reference to the list of known not-affected product IDs.
    fn get_known_not_affected(&self) -> Option<Vec<&String>>;

    /// Returns a reference to the list of last affected product IDs.
    fn get_last_affected(&self) -> Option<Vec<&String>>;

    /// Returns a reference to the list of recommended product IDs.
    fn get_recommended(&self) -> Option<Vec<&String>>;

    /// Returns a reference to the list of product IDs currently under investigation.
    fn get_under_investigation(&self) -> Option<Vec<&String>>;

    /// Combines all affected product IDs into a `HashSet`.
    ///
    /// This method aggregates product IDs from these lists:
    /// - First affected product IDs
    /// - Last affected product IDs
    /// - Known affected product IDs
    ///
    /// # Returns
    ///
    /// A `HashSet` containing all aggregated product IDs. If none of these lists are
    /// populated, the returned `HashSet` will be empty.
    fn get_all_affected(&self) -> HashSet<&String> {
        let mut result = HashSet::new();

        if let Some(first_affected) = self.get_first_affected() {
            result.extend(first_affected);
        }

        if let Some(last_affected) = self.get_last_affected() {
            result.extend(last_affected);
        }

        if let Some(known_affected) = self.get_known_affected() {
            result.extend(known_affected);
        }

        result
    }

    /// Combines all fixed product IDs into a `HashSet`.
    ///
    /// This method aggregates product IDs from these lists:
    /// - First fixed product IDs
    /// - Fixed product IDs
    ///
    /// # Returns
    ///
    /// A `HashSet` containing all aggregated product IDs. If none of these lists are
    /// populated, the returned `HashSet` will be empty.
    fn get_all_fixed(&self) -> HashSet<&String> {
        let mut result = HashSet::new();

        if let Some(first_fixed) = self.get_first_fixed() {
            result.extend(first_fixed);
        }

        if let Some(fixed) = self.get_fixed() {
            result.extend(fixed);
        }

        result
    }
}

/// Trait representing an abstract metric in a CSAF document.
pub trait MetricTrait {
    /// Retrieves a vector of product IDs associated with this metric.
    fn get_products(&self) -> Vec<&String>;
}

/// Trait representing an abstract threat in a CSAF document.
pub trait ThreatTrait {
    /// Retrieves a list of product IDs associated with this threat, if any.
    fn get_product_ids(&self) -> Option<Vec<&String>>;

    /// Returns the date associated with this threat
    fn get_date(&self) -> &Option<String>;
}

/// Trait representing an abstract product tree in a CSAF document.
///
/// The `ProductTreeTrait` defines the structure of a product tree and allows
/// access to its product groups.
pub trait ProductTreeTrait {
    /// The associated type representing the type of branch in the product tree.
    type BranchType: BranchTrait;

    /// The associated type representing the type of product groups in the product tree.
    type ProductGroupType: ProductGroupTrait;

    /// The associated type representing the type of relationships in the product tree.
    type RelationshipType: RelationshipTrait;

    /// The associated type representing the type of full product name.
    type FullProductNameType: FullProductNameTrait;

    /// Returns an optional reference to the list of branches in the product tree.
    fn get_branches(&self) -> Option<&Vec<Self::BranchType>>;

    /// Retrieves a reference to the list of product groups in the product tree.
    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType>;

    /// Retrieves a reference to the list of relationships in the product tree.
    fn get_relationships(&self) -> &Vec<Self::RelationshipType>;

    /// Retrieves a reference to the list of full product names in the product tree.
    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType>;
}

/// Trait representing an abstract branch in a product tree.
pub trait BranchTrait {
    /// The associated type representing child branches.
    type BranchType: BranchTrait;

    /// The associated type representing a full product name.
    type FullProductNameType: FullProductNameTrait;

    /// Returns an optional reference to the child branches of this branch.
    fn get_branches(&self) -> Option<&Vec<Self::BranchType>>;

    /// Retrieves the full product name associated with this branch, if available.
    fn get_product(&self) -> Option<&Self::FullProductNameType>;
}

/// Trait representing an abstract product group in a CSAF document.
///
/// The `ProductGroupTrait` encapsulates the details of a product group, including
/// its IDs and associated product IDs.
pub trait ProductGroupTrait {
    /// Returns the unique identifier of the product group.
    fn get_group_id(&self) -> &String;

    /// Retrieves a vector of product IDs contained within the product group.
    fn get_product_ids(&self) -> Vec<&String>;
}

/// Trait representing an abstract relationship in a product tree.
pub trait RelationshipTrait {
    /// The associated type representing a full product name.
    type FullProductNameType: FullProductNameTrait;

    /// Retrieves the product reference identifier.
    fn get_product_reference(&self) -> &String;

    /// Retrieves the identifier of the related product.
    fn get_relates_to_product_reference(&self) -> &String;

    /// Retrieves the full product name associated with the relationship.
    fn get_full_product_name(&self) -> &Self::FullProductNameType;
}

/// Trait representing an abstract full product name in a CSAF document.
pub trait FullProductNameTrait {
    /// Returns the product ID from the full product name.
    fn get_product_id(&self) -> &String;
}
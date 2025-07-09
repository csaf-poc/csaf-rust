use crate::csaf::csaf2_1::schema::{CategoryOfTheRemediation, DocumentStatus, Epss, LabelOfTlp};
use crate::csaf::csaf2_1::ssvc_schema::SsvcV1;
use crate::csaf::helpers::resolve_product_groups;
use crate::csaf::validation::ValidationError;
use std::collections::{BTreeSet, HashSet};

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
    fn get_product_tree(&self) -> &Option<Self::ProductTreeType>;

    /// Retrieves all vulnerabilities present in the CSAF document.
    fn get_vulnerabilities(&self) -> &Vec<Self::VulnerabilityType>;

    /// Retrieves the document meta present in the CSAF document.
    fn get_document(&self) -> &Self::DocumentType;
}

/// Trait representing document meta level information
pub trait DocumentTrait {
    /// Type representing document tracking information
    type TrackingType: TrackingTrait;

    /// Type representing document distribution information
    type DistributionType: DistributionTrait;

    /// Type representing document notes
    type NoteType: NoteTrait;

    /// Returns the tracking information for this document
    fn get_tracking(&self) -> &Self::TrackingType;

    /// Returns the distribution information for this document with CSAF 2.1 semantics
    fn get_distribution_21(&self) -> Result<&Self::DistributionType, ValidationError>;

    /// Returns the distribution information for this document with CSAF 2.0 semantics
    fn get_distribution_20(&self) -> Option<&Self::DistributionType>;

    /// Returns the notes associtated with this document
    fn get_notes(&self) -> Option<&Vec<Self::NoteType>>;
}

/// Trait representing distribution information for a document
pub trait DistributionTrait {
    /// Type representing sharing group information
    type SharingGroupType: SharingGroupTrait;

    /// Type representing TLP (Traffic Light Protocol) information
    type TlpType: TlpTrait;

    /// Returns the sharing group for this distribution
    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType>;

    /// Returns the TLP information for this distribution with CSAF 2.0 semantics
    fn get_tlp_20(&self) -> Option<&Self::TlpType>;

    /// Returns the TLP information for this distribution with CSAF 2.1 semantics
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError>;
}

pub trait NoteTrait: WithGroupIds {}

/// Trait representing sharing group information
pub trait SharingGroupTrait {
    /// Returns the ID of the sharing group
    fn get_id(&self) -> &String;

    /// Returns the optional name of the sharing group
    fn get_name(&self) -> Option<&String>;
}

/// Trait representing TLP (Traffic Light Protocol) information
pub trait TlpTrait {
    /// Returns the TLP label
    fn get_label(&self) -> LabelOfTlp;
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

    /// Returns the status of this document
    fn get_status(&self) -> DocumentStatus;

    /// Returns the tracking ID of this document
    fn get_id(&self) -> &String;
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

    /// The associated type representing a vulnerability flag.
    type FlagType: FlagTrait;

    /// The associated type representing a vulnerability involvement.
    type InvolvementType: InvolvementTrait;

    /// The associated type representing the vulnerability ID information.
    type VulnerabilityIdType: VulnerabilityIdTrait;

    /// The associated type representing vulnerability notes.
    type NoteType: NoteTrait;

    /// Retrieves a list of remediations associated with the vulnerability.
    fn get_remediations(&self) -> &Vec<Self::RemediationType>;

    /// Retrieves the status of products affected by the vulnerability, if available.
    fn get_product_status(&self) -> &Option<Self::ProductStatusType>;

    /// Returns an optional vector of metrics related to the vulnerability.
    fn get_metrics(&self) -> Option<&Vec<Self::MetricType>>;

    /// Retrieves a list of potential threats related to the vulnerability.
    fn get_threats(&self) -> &Vec<Self::ThreatType>;

    /// Returns the date when this vulnerability was initially disclosed.
    fn get_disclosure_date(&self) -> &Option<String>;

    /// Returns the date when this vulnerability was initially discovered.
    fn get_discovery_date(&self) -> &Option<String>;

    /// Returns all flags associated with this vulnerability.
    fn get_flags(&self) -> &Option<Vec<Self::FlagType>>;

    /// Returns all involvements associated with this vulnerability.
    fn get_involvements(&self) -> &Option<Vec<Self::InvolvementType>>;

    /// Returns the CVE associated with the vulnerability.
    fn get_cve(&self) -> Option<&String>;

    /// Returns the vulnerability IDs associated with this vulnerability.
    fn get_ids(&self) -> &Option<Vec<Self::VulnerabilityIdType>>;

    /// Returns the notes associated with this vulnerability.
    fn get_notes(&self) -> Option<&Vec<Self::NoteType>>;
}

pub trait VulnerabilityIdTrait {
    fn get_system_name(&self) -> &String;

    fn get_text(&self) -> &String;
}

/// Trait for accessing vulnerability flags information
pub trait FlagTrait: WithGroupIds {
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
pub trait RemediationTrait: WithGroupIds {
    /// Returns the category of the remediation.
    ///
    /// Categories are defined by the CSAF schema.
    fn get_category(&self) -> CategoryOfTheRemediation;

    /// Retrieves the product IDs directly affected by this remediation, if any.
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;

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
                Some(product_ids) => product_ids.map(|id| (*id).to_owned()).collect(),
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
    fn get_first_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of first fixed product IDs.
    fn get_first_fixed(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of fixed product IDs.
    fn get_fixed(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of known affected product IDs.
    fn get_known_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of known not-affected product IDs.
    fn get_known_not_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of last affected product IDs.
    fn get_last_affected(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of recommended product IDs.
    fn get_recommended(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns a reference to the list of product IDs currently under investigation.
    fn get_under_investigation(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Return a reference to the list of product IDs with unknown status.
    fn get_unknown(&self) -> Option<impl Iterator<Item = &String> + '_>;

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
    type ContentType: ContentTrait;

    /// Retrieves a vector of product IDs associated with this metric.
    fn get_products(&self) -> impl Iterator<Item = &String> + '_;

    fn get_content(&self) -> &Self::ContentType;

    fn get_source(&self) -> &Option<String>;
}

pub trait ContentTrait {
    fn has_ssvc_v1(&self) -> bool;

    fn get_ssvc_v1(&self) -> Result<SsvcV1, serde_json::Error>;

    fn get_cvss_v2(&self) -> Option<&serde_json::Map<String, serde_json::Value>>;

    fn get_cvss_v3(&self) -> Option<&serde_json::Map<String, serde_json::Value>>;

    fn get_cvss_v4(&self) -> Option<&serde_json::Map<String, serde_json::Value>>;

    fn get_epss(&self) -> &Option<Epss>;

    fn get_content_json_path(&self, vulnerability_idx: usize, metric_idx: usize) -> String;
}

/// Trait representing an abstract threat in a CSAF document.
pub trait ThreatTrait: WithGroupIds {
    /// Retrieves a list of product IDs associated with this threat, if any.
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;

    /// Returns the date associated with this threat
    fn get_date(&self) -> &Option<String>;
}

/// Trait representing an abstract product tree in a CSAF document.
///
/// The `ProductTreeTrait` defines the structure of a product tree and allows
/// access to its product groups.
pub trait ProductTreeTrait {
    /// The associated type representing the type of branch in the product tree.
    type BranchType: BranchTrait<Self::FullProductNameType>;

    /// The associated type representing the type of product groups in the product tree.
    type ProductGroupType: ProductGroupTrait;

    /// The associated type representing the type of relationships in the product tree.
    type RelationshipType: RelationshipTrait<Self::FullProductNameType>;

    /// The associated type representing the type of full product name.
    type FullProductNameType: ProductTrait;

    /// Returns an optional reference to the list of branches in the product tree.
    fn get_branches(&self) -> Option<&Vec<Self::BranchType>>;

    /// Retrieves a reference to the list of product groups in the product tree.
    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType>;

    /// Retrieves a reference to the list of relationships in the product tree.
    fn get_relationships(&self) -> &Vec<Self::RelationshipType>;

    /// Retrieves a reference to the list of full product names in the product tree.
    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType>;

    /// Visits all product references in the product tree by invoking the provided callback for each
    /// product. Returns immediately with the error Result provided by `callback`, if occurring.
    ///
    /// This method traverses all locations in the product tree where products can be referenced:
    /// - Products within branches (recursively)
    /// - Full product names at the top level
    /// - Full product names within relationships
    ///
    /// # Parameters
    /// * `callback` - A mutable function that takes a reference to a product and its path string,
    ///   and returns a `Result<(), ValidationError>`. The path string represents the JSON pointer
    ///   to the product's location in the document.
    ///
    /// # Returns
    /// * `Ok(())` if all products were visited successfully
    /// * `Err(ValidationError)` if the callback returned an error for any product
    fn visit_all_products_generic(
        &self,
        callback: &mut impl FnMut(&Self::FullProductNameType, &str) -> Result<(), ValidationError>,
    ) -> Result<(), ValidationError> {
        // Visit products in branches
        if let Some(branches) = self.get_branches().as_ref() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_branches_rec(
                    &format!("/product_tree/branches/{}", i),
                    &mut |branch: &Self::BranchType, path| {
                        if let Some(product_ref) = branch.get_product() {
                            callback(product_ref, &format!("{}/product", path))?;
                        }
                        Ok(())
                    },
                )?;
            }
        }

        // Visit full_product_names
        for (i, fpn) in self.get_full_product_names().iter().enumerate() {
            callback(fpn, &format!("/product_tree/full_product_names/{}", i))?;
        }

        // Visit relationships
        for (i, rel) in self.get_relationships().iter().enumerate() {
            callback(
                rel.get_full_product_name(),
                &format!("/product_tree/relationships/{}/full_product_name", i),
            )?;
        }

        Ok(())
    }

    /// A trait wrapper for `visit_all_products_generic()` that allows implementations to provide
    /// type-specific callbacks for product traversal.
    ///
    /// This method is intended to be implemented by trait objects to handle their specific
    /// product name types while reusing the generic traversal logic defined in
    /// `visit_all_products_generic()`.
    ///
    /// # Parameters
    /// * `callback` - A mutable function that takes a reference to a product and its path string,
    ///   returning a `Result<(), ValidationError>`. The callback will be invoked with the concrete
    ///   type specified by the implementing trait.
    ///
    /// # Returns
    /// * `Ok(())` if all products were visited successfully
    /// * `Err(ValidationError)` if the callback returned an error for any product
    ///
    /// # Implementation Note
    /// Trait implementers should typically implement this by delegating to
    /// `visit_all_products_generic()` with the same callback.
    fn visit_all_products(
        &self,
        callback: &mut impl FnMut(&Self::FullProductNameType, &str) -> Result<(), ValidationError>,
    ) -> Result<(), ValidationError>;
}

/// Trait representing an abstract branch in a product tree.
pub trait BranchTrait<FPN: ProductTrait>: Sized {
    /// Returns an optional reference to the child branches of this branch.
    fn get_branches(&self) -> Option<&Vec<Self>>;

    /// Retrieves the full product name associated with this branch, if available.
    fn get_product(&self) -> &Option<FPN>;

    /// Recursively visits all branches in the tree structure,
    /// applying the provided callback function to each branch.
    ///
    /// This method traverses the entire branch hierarchy, starting from the current branch and
    /// proceeding depth-first through all child branches. For each branch, it calls the
    /// provided callback function with the branch object and its path representation.
    ///
    /// # Parameters
    /// * `path` - A string representing the current path in the branch hierarchy
    /// * `callback` - A mutable function that takes a reference to Self and the
    ///   current path string, and returns a Result
    ///
    /// # Returns
    /// * `Ok(())` if the traversal completes successfully
    /// * `Err(ValidationError)` if the callback returns an error for any branch
    fn visit_branches_rec(
        &self,
        path: &str,
        callback: &mut impl FnMut(&Self, &str) -> Result<(), ValidationError>,
    ) -> Result<(), ValidationError> {
        callback(self, path)?;
        if let Some(branches) = self.get_branches().as_ref() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_branches_rec(&format!("{}/branches/{}", path, i), callback)?;
            }
        }
        Ok(())
    }

    /// Searches for branches that exceed the maximum allowed depth in the branch hierarchy.
    ///
    /// This method recursively checks if the branch structure exceeds the specified depth limit.
    /// It traverses the branch hierarchy depth-first, decrementing the remaining depth parameter
    /// at each level. If branches are found beyond the allowed depth, it returns the path to the
    /// first excessive branch.
    ///
    /// # Parameters
    /// * `remaining_depth` - The maximum number of branch levels still allowed
    ///
    /// # Returns
    /// * `Some(String)` containing the path to the first branch that exceeds the allowed depth
    /// * `None` if no branches exceed the allowed depth
    fn find_excessive_branch_depth(&self, remaining_depth: u32) -> Option<String> {
        if let Some(branches) = self.get_branches() {
            // If we've reached depth limit and there are branches, we've found a violation
            if remaining_depth == 1 {
                return Some("/branches/0".to_string());
            }
            for (i, branch) in branches.iter().enumerate() {
                if let Some(sub_path) = branch.find_excessive_branch_depth(remaining_depth - 1) {
                    return Some(format!("/branches/{}{}", i, sub_path));
                }
            }
        }
        None
    }
}

/// Trait representing an abstract product group in a CSAF document.
///
/// The `ProductGroupTrait` encapsulates the details of a product group, including
/// its IDs and associated product IDs.
pub trait ProductGroupTrait {
    /// Returns the unique identifier of the product group.
    fn get_group_id(&self) -> &String;

    /// Retrieves a vector of product IDs contained within the product group.
    fn get_product_ids(&self) -> impl Iterator<Item = &String> + '_;
}

/// Trait representing an abstract relationship in a product tree.
pub trait RelationshipTrait<FPN: ProductTrait> {
    /// Retrieves the product reference identifier.
    fn get_product_reference(&self) -> &String;

    /// Retrieves the identifier of the related product.
    fn get_relates_to_product_reference(&self) -> &String;

    /// Retrieves the full product name associated with the relationship.
    fn get_full_product_name(&self) -> &FPN;
}

/// Trait representing an abstract full product name in a CSAF document.
pub trait ProductTrait {
    /// The associated type representing a product identification helper.
    type ProductIdentificationHelperType: ProductIdentificationHelperTrait;

    /// Returns the product ID from the full product name.
    fn get_product_id(&self) -> &String;

    /// Returns the product identification helper associated with the full product name.
    fn get_product_identification_helper(&self) -> &Option<Self::ProductIdentificationHelperType>;
}

/// Trait representing an abstract product identification helper of a full product name.
pub trait ProductIdentificationHelperTrait {
    /// Returns the PURLs identifying the associated product.
    fn get_purls(&self) -> Option<&[String]>;

    fn get_model_numbers(&self) -> Option<impl Iterator<Item = &String> + '_>;

    fn get_serial_numbers(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

pub trait WithGroupIds {
    /// Returns the product group IDs associated with this vulnerability flag
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

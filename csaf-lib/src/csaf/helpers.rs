use crate::csaf::getter_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use std::collections::BTreeSet;

pub fn resolve_product_groups(doc: &impl CsafTrait, product_groups: Vec<&String>) -> Option<BTreeSet<String>> {
    doc.get_product_tree().map(|product_tree| {
        product_tree
            .get_product_groups()
            .iter()
            .filter(|x| product_groups.iter().any(|g| *g == x.get_group_id()))
            .map(|x| x.get_product_ids().iter().map(|p| p.to_string()).collect::<Vec<String>>())
            .flatten()
            .collect()
    })
}
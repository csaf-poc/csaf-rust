use crate::csaf::getter_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use std::collections::BTreeSet;

pub fn resolve_product_groups<'a, I>(doc: &impl CsafTrait, product_groups: I) -> Option<BTreeSet<String>>
where
    I: IntoIterator<Item = &'a String>
{
    let product_groups: Vec<&String> = product_groups.into_iter().collect();

    doc.get_product_tree().as_ref().map(|product_tree| {
        product_tree
            .get_product_groups()
            .iter()
            .filter(|x| product_groups.iter().any(|g| *g == x.get_group_id()))
            .map(|x| x.get_product_ids().map(|p| p.to_string()).collect::<Vec<String>>())
            .flatten()
            .collect()
    })
}

/// Counts the number of unescaped '*' characters in a given string.
/// An asterisk is considered "unescaped" if it is not preceded by a backslash ('\\').
/// Consecutive backslashes alternate between escaping or not escaping characters.
///
/// # Arguments
///
/// * `s` - A string slice to be analyzed.
///
/// # Returns
///
/// Returns the number of unescaped '*' characters found in the string.
pub fn count_unescaped_stars(s: &str) -> u32 {
    let mut escaped = false;
    let mut count = 0u32;
    for c in s.chars() {
        match c {
            '\\' => escaped = !escaped,
            '*' if !escaped => count += 1,
            _ => escaped = false,
        }
    }
    count
}

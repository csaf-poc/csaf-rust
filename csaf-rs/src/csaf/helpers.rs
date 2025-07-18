use crate::csaf::csaf2_1::ssvc_dp_schema::DecisionPoint;
use crate::csaf::getter_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use rust_embed::RustEmbed;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::Deref;
use std::sync::LazyLock;
use uuid::{uuid, Uuid};

/// Special name for public sharing groups
pub static SG_NAME_PUBLIC: &str = "Public";
/// Special name for private sharing groups
pub static SG_NAME_PRIVATE: &str = "No sharing allowed";

/// Special "max" UUID value
pub static MAX_UUID: &Uuid = &uuid!("ffffffff-ffff-ffff-ffff-ffffffffffff");
/// Special "nil" UUID value
pub static NIL_UUID: &Uuid = &uuid!("00000000-0000-0000-0000-000000000000");

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

#[derive(RustEmbed)]
#[folder = "assets/ssvc_decision_points/"]
#[include = "*.json"]
struct SsvcDecisionPointJsonFiles;

/// Recursively loads all decision point JSON descriptions from `../ssvc/data/json/decision_points`.
/// Entries are stored in a `HashMap` indexed by their respective (name, version) tuple for lookup.
pub static SSVC_DECISION_POINTS: LazyLock<HashMap<(String, String, String), DecisionPoint>> = LazyLock::new(|| {
    let mut decision_points = HashMap::new();

    for filename in SsvcDecisionPointJsonFiles::iter() {
        if let Some(file) = SsvcDecisionPointJsonFiles::get(&filename) {
            let content = std::str::from_utf8(&file.data).unwrap();
            match serde_json::from_str::<DecisionPoint>(content) {
                Ok(dp) => {
                    println!("Loaded SSVC decision point '{}' (version {})", dp.name.deref(), dp.version.deref());
                    let key = (
                        dp.namespace.deref().to_owned(),
                        dp.name.deref().to_owned(),
                        dp.version.deref().to_owned(),
                    );
                    decision_points.insert(key, dp);
                },
                Err(err) => eprintln!("Warning: Failed to parse decision point from file {}: {}", filename, err),
            }
        }
    }

    decision_points
});

/// Derives lookup maps for all observed SSVC decision points that can be used
/// to verify the order of values within the respective decision points.
pub static DP_VAL_LOOKUP: LazyLock<HashMap<(String, String, String), HashMap<String, i32>>> = LazyLock::new(|| {
    let mut lookups = HashMap::new();

    for (key, dp) in SSVC_DECISION_POINTS.iter() {
        let mut lookup_map = HashMap::new();
        for (i, v) in dp.values.iter().enumerate() {
            lookup_map.insert(v.name.deref().to_owned(), i as i32);
        }
        lookups.insert(key.clone(), lookup_map);
    }

    lookups
});

/// Collects all "registered" namespaces from known decision points. We assume that each namespace
/// that occurs in at least one decision point in the SSVC repository is a "registered" namespace.
pub static REGISTERED_SSVC_NAMESPACES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut namespaces = HashSet::new();

    for (namespace, _, _) in SSVC_DECISION_POINTS.keys() {
        namespaces.insert(namespace.to_owned());
    }

    namespaces
});
use crate::csaf::getter_traits::{BranchTrait, CsafTrait, ProductTreeTrait};
use crate::csaf::validation::ValidationError;

static MAX_DEPTH: u32 = 30;

pub fn test_6_1_34_branches_recursion_depth(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    if let Some(tree) = doc.get_product_tree().as_ref() {
        if let Some(branches) = tree.get_branches() {
            for (i, branch) in branches.iter().enumerate() {
                if let Some(path) = branch.find_excessive_branch_depth(MAX_DEPTH) {
                    return Err(ValidationError {
                        message: format!("Branches recursion depth too big (> {})", MAX_DEPTH),
                        instance_path: format!("/product_tree/branches/{}{}", i, path)
                    });
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_34::test_6_1_34_branches_recursion_depth;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_34() {
        run_csaf21_tests(
            "34",
            test_6_1_34_branches_recursion_depth,
            HashMap::from([
                ("01", &ValidationError {
                    message: "Branches recursion depth too big (> 30)".to_string(),
                    instance_path: "/product_tree/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0".to_string(),
                }),
                ("02", &ValidationError {
                    message: "Branches recursion depth too big (> 30)".to_string(),
                    instance_path: "/product_tree/branches/0/branches/0/branches/1/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0/branches/0\
                    /branches/0/branches/0/branches/0/branches/0/branches/0/branches/0".to_string(),
                }),
            ]),
        );
    }
}

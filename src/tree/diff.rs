use crate::tree::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum DiffStatus {
    Added,
    Removed,
    Modified,
    Unchanged,
}

#[derive(Debug, Clone)]
pub struct DiffNode {
    pub name: String,

    pub old_value: Option<String>,
    pub new_value: Option<String>,

    pub status: DiffStatus,

    pub children: Vec<DiffNode>,
}

pub fn diff_trees(
    source: Option<&Node>,
    target: Option<&Node>,
) -> DiffNode {

    match (source, target) {

        // ADDED
        (None, Some(target_node)) => {

            DiffNode {
                name: target_node.name.clone(),

                old_value: None,
                new_value: target_node.value.clone(),

                status: DiffStatus::Added,

                children: target_node
                    .children
                    .iter()
                    .map(|child| {
                        diff_trees(None, Some(child))
                    })
                    .collect(),
            }
        }

        // REMOVED
        (Some(source_node), None) => {

            DiffNode {
                name: source_node.name.clone(),

                old_value: source_node.value.clone(),
                new_value: None,

                status: DiffStatus::Removed,

                children: source_node
                    .children
                    .iter()
                    .map(|child| {
                        diff_trees(Some(child), None)
                    })
                    .collect(),
            }
        }

        // BOTH EXIST
        (Some(source_node), Some(target_node)) => {

            let status =
                if source_node.value != target_node.value {
                    DiffStatus::Modified
                } else {
                    DiffStatus::Unchanged
                };

            let mut children = vec![];

            // compare source children
            for source_child in &source_node.children {

                let target_match =
                    target_node
                        .children
                        .iter()
                        .find(|t| t.name == source_child.name);

                children.push(
                    diff_trees(
                        Some(source_child),
                        target_match,
                    )
                );
            }

            // added nodes
            for target_child in &target_node.children {

                let exists_in_source =
                    source_node
                        .children
                        .iter()
                        .any(|s| s.name == target_child.name);

                if !exists_in_source {

                    children.push(
                        diff_trees(
                            None,
                            Some(target_child),
                        )
                    );
                }
            }

            DiffNode {
                name: source_node.name.clone(),

                old_value: source_node.value.clone(),
                new_value: target_node.value.clone(),

                status,

                children,
            }
        }

        _ => unreachable!(),
    }
}

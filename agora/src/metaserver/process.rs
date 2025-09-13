use super::pathtree::{TreeNode, TreeTrait};
use std::collections::HashMap;

type Publisher = String;

pub struct Process {
    path_tree: TreeNode,
    publishers: HashMap<String, Publisher>,
    port: u16,
}

// impl Process {
//     pub fn new(name: impl Into<String>, port: u16) -> Self {
//         Self {
//             path_tree: *TreeNode::new(name),
//             publishers: HashMap::new(),
//             port: port,
//         }
//     }

//     pub fn add_publisher(self: &mut Self, name: impl Into<String>, path:) {
// }

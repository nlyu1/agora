use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct TreeNode {
    name: String,
    children: RefCell<Vec<Rc<TreeNode>>>,
    parent: RefCell<Option<Weak<TreeNode>>>,
}

type TreeNodeRef = Rc<TreeNode>;

pub trait TreeTrait {
    fn new(name: impl Into<String>) -> Rc<Self>;
    fn add_children(self: &Rc<Self>, names: &[&str]);
    fn add_child(self: &Rc<Self>, child: TreeNodeRef);
    fn get_child(self: &Rc<Self>, path: &str) -> Option<TreeNodeRef>;
    fn remove_child(self: &Rc<Self>, name: &str) -> Result<(), &'static str>;
    fn parent(&self) -> Option<TreeNodeRef>;
    fn children(&self) -> Vec<TreeNodeRef>;
    fn name(&self) -> &str;
    fn path(&self) -> String;
    fn is_root(&self) -> bool;
    fn is_leaf(&self) -> bool;
    fn display_tree(&self) -> String;
}

impl TreeNode {
    fn to_string_helper(&self, prefix: &str, is_last: bool) -> String {
        let mut result = String::new();

        // Add current node
        let connector = if is_last { "└── " } else { "├── " };
        result.push_str(&format!("{}{}{}\n", prefix, connector, self.name));

        // Prepare prefix for children
        let child_prefix = if is_last {
            format!("{}    ", prefix) // Four spaces for last items
        } else {
            format!("{}│   ", prefix) // Vertical bar + three spaces for non-last items
        };

        let children = self.children.borrow();
        let child_count = children.len();

        for (i, child) in children.iter().enumerate() {
            let is_last_child = i == child_count - 1;
            result.push_str(&child.to_string_helper(&child_prefix, is_last_child));
        }

        result
    }

    fn get_immediate_child(self: &Rc<Self>, name: &str) -> Option<TreeNodeRef> {
        // Returns child if exists. Note that modifying child will modify original tree.
        // Change type annotation as necessary to complete this functionality.
        self.children
            .borrow()
            .iter()
            .find(|child| child.name == name)
            .cloned()
    }
}

impl TreeTrait for TreeNode {
    fn new(name: impl Into<String>) -> Rc<Self> {
        Rc::new(TreeNode {
            name: name.into(),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        })
    }

    fn add_children(self: &Rc<Self>, names: &[&str]) {
        // Create children of names as specified; set their parents correctly
        for name in names {
            let child = TreeNode::new(*name);
            // Set parent of child
            *child.parent.borrow_mut() = Some(Rc::downgrade(self));
            // Add child to this node
            self.children.borrow_mut().push(child);
        }
    }

    fn add_child(self: &Rc<Self>, child: TreeNodeRef) {
        // Set parent of child
        *child.parent.borrow_mut() = Some(Rc::downgrade(self));
        // Add child to this node
        self.children.borrow_mut().push(child);
    }

    fn get_child(self: &Rc<Self>, path: &str) -> Option<TreeNodeRef> {
        // Same as "get_child", except might be recursive child1/child2/...
        if path.is_empty() {
            return Some(self.clone());
        }

        let path_parts: Vec<&str> = path.split('/').collect();
        let mut current_node = self.clone();

        for part in path_parts {
            if part.is_empty() {
                continue; // Skip empty parts (e.g., from leading/trailing slashes)
            }

            current_node = current_node.get_immediate_child(part)?;
        }

        Some(current_node)
    }

    fn remove_child(self: &TreeNodeRef, name: &str) -> Result<(), &'static str> {
        // Look for "name" if exists and deletes; frees correctly. Else complains [name] not found under [self.path]
        let mut children = self.children.borrow_mut();
        let initial_len = children.len();

        children.retain(|child| child.name != name);

        if children.len() == initial_len {
            Err("Child not found")
        } else {
            Ok(())
        }
    }
    fn parent(&self) -> Option<TreeNodeRef> {
        self.parent.borrow().as_ref()?.upgrade()
    }

    fn children(&self) -> Vec<TreeNodeRef> {
        self.children.borrow().clone()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> String {
        if self.is_root() {
            format!("/{}", self.name().to_string())
        } else {
            format!("{}/{}", self.parent().unwrap().path(), self.name())
        }
    }

    fn is_root(&self) -> bool {
        self.parent.borrow().is_none()
    }

    fn is_leaf(&self) -> bool {
        self.children.borrow().is_empty()
    }

    fn display_tree(&self) -> String {
        self.to_string_helper("", true)
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Remove the trailing newline for cleaner display formatting
        let tree_str = self.display_tree();
        write!(f, "{}", tree_str.trim_end())
    }
}

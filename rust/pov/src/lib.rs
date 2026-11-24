use std::collections::BTreeSet;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: BTreeSet<Tree<T>>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            label,
            children: BTreeSet::new(),
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.insert(child);
        self
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        let mut ancestors: Vec<Tree<T>> = Vec::new();

        let found = self.collect_ancestors(from, &mut ancestors);

        if let Some(mut subtree) = ancestors.pop() {
            while let Some(mut parent) = ancestors.pop() {
                parent.children.insert(subtree);
                subtree = parent;
            }
            self.children.insert(subtree);
        }
        found
    }
    fn collect_ancestors(&mut self, from: &T, ancestors: &mut Vec<Tree<T>>) -> bool {
        if &self.label == from {
            return true;
        };

        let org_children = std::mem::take(&mut self.children);
        let mut child_found = None;

        for mut child in org_children {
            if child_found.is_none() && child.collect_ancestors(from, ancestors) {
                child_found = Some(child);
            } else {
                self.children.insert(child);
            }
        }
        if let Some(mut new_root) = child_found {
            std::mem::swap(self, &mut new_root);
            ancestors.push(new_root);
            true
        } else {
            false
        }
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(from) {
            return None;
        }
        let mut path = self.path_to(to)?;
        path.reverse();
        Some(path)
    }

    pub fn path_to<'a>(&'a self, to: &'a T) -> Option<Vec<&'a T>> {
        if &self.label == to {
            return Some(vec![&self.label]);
        };

        for child in &self.children {
            if let Some(mut path) = child.path_to(to) {
                path.push(&self.label);
                return Some(path);
            }
        }
        None
    }
}

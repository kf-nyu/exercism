use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Hash + Eq> {
    elements: HashSet<T>,
}

impl<T: Hash + Eq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self {
            elements: input.iter().cloned().collect(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.elements.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.is_subset(&other.elements)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.elements.is_disjoint(&other.elements)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let intersection: Vec<T> = self
            .elements
            .intersection(&other.elements)
            .cloned()
            .collect();
        CustomSet::new(&intersection)
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let difference: Vec<T> = self.elements.difference(&other.elements).cloned().collect();
        CustomSet::new(&difference)
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let union: Vec<T> = self.elements.union(&other.elements).cloned().collect();
        CustomSet::new(&union)
    }
}

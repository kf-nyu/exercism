use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Default)]
pub struct School(HashMap<String, u32>);

impl School {
    pub fn new() -> School {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0.entry(student.to_string()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0
            .values()
            .copied()
            .collect::<BTreeSet<u32>>()
            .into_iter()
            .collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0
            .iter()
            .filter(|&(_name, &val)| val == grade)
            .map(|(name, &_val)| name.clone())
            .collect::<BTreeSet<String>>()
            .into_iter()
            .collect()
    }
}

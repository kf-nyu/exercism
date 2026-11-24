#[derive(Debug)]
pub struct Allergies {
    allergies: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { allergies: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        println!("{:?}", self);
        self.allergies & (*allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut v = Vec::new();
        if self.allergies & 1 != 0 {
            v.push(Allergen::Eggs)
        };
        if self.allergies & 2 != 0 {
            v.push(Allergen::Peanuts)
        };
        if self.allergies & 4 != 0 {
            v.push(Allergen::Shellfish)
        };
        if self.allergies & 8 != 0 {
            v.push(Allergen::Strawberries)
        };
        if self.allergies & 16 != 0 {
            v.push(Allergen::Tomatoes)
        };
        if self.allergies & 32 != 0 {
            v.push(Allergen::Chocolate)
        };
        if self.allergies & 64 != 0 {
            v.push(Allergen::Pollen)
        };
        if self.allergies & 128 != 0 {
            v.push(Allergen::Cats)
        };

        v
    }
}

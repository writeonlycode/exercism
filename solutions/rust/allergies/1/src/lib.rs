pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        use Allergen::*;

        match allergen {
            Eggs => self.score & (1 << 0) != 0,
            Peanuts => self.score & (1 << 1) != 0,
            Shellfish => self.score & (1 << 2) != 0,
            Strawberries => self.score & (1 << 3) != 0,
            Tomatoes => self.score & (1 << 4) != 0,
            Chocolate => self.score & (1 << 5) != 0,
            Pollen => self.score & (1 << 6) != 0,
            Cats => self.score & (1 << 7) != 0,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;

        let mut allergies: Vec<Allergen> = vec![];

        if self.score & (1 << 0) != 0 { allergies.push(Eggs); }
        if self.score & (1 << 1) != 0 { allergies.push(Peanuts); }
        if self.score & (1 << 2) != 0 { allergies.push(Shellfish); }
        if self.score & (1 << 3) != 0 { allergies.push(Strawberries); }
        if self.score & (1 << 4) != 0 { allergies.push(Tomatoes); }
        if self.score & (1 << 5) != 0 { allergies.push(Chocolate); }
        if self.score & (1 << 6) != 0 { allergies.push(Pollen); }
        if self.score & (1 << 7) != 0 { allergies.push(Cats); }

        allergies
    }
}

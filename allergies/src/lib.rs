pub struct Allergies {
    score: u32,
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
        let mut allergens: Vec<Allergen> = vec![];
        for mask in 0_u32..=7 {
            let mask_value = score & (1 << mask);
            match (mask, mask_value > 0) {
                (0, true) => allergens.push(Allergen::Eggs),
                (1, true) => allergens.push(Allergen::Peanuts),
                (2, true) => allergens.push(Allergen::Shellfish),
                (3, true) => allergens.push(Allergen::Strawberries),
                (4, true) => allergens.push(Allergen::Tomatoes),
                (5, true) => allergens.push(Allergen::Chocolate),
                (6, true) => allergens.push(Allergen::Pollen),
                (7, true) => allergens.push(Allergen::Cats),
                _ => {} // Just Ignore
            }
        }
        Allergies { score, allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.to_vec()
    }

}

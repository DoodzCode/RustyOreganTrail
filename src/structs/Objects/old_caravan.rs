use super::{
    clothing::{Attire, Clothing, WearLocation},
    material::Material,
};

#[derive(Debug)]
pub struct Caravan {
    pub population: u8,
    pub supplies: Supplies,
    pub population_attire: Attire,
    pub clothing_storage: Vec<Clothing>,
}

impl Caravan {
    pub fn new() -> Caravan {
        //! Fix this hardcoded bullshit
        Caravan {
            population: 8, 
            supplies: Supplies::new("low"),
            population_attire: Attire {
                head: Clothing::new("Fur Hat", WearLocation::Head, Material::Cotton),
                chest: Clothing::new("Fur Shirt", WearLocation::Chest, Material::Cotton),
                legs: Clothing::new("Fur Pants", WearLocation::Legs, Material::Cotton),
                feet: Clothing::new("Fur Slippers", WearLocation::Feet, Material::Cotton),
                hands: Clothing::blank(),
            },
            clothing_storage: Vec::new(),
        }
    }

    pub fn store_clothing(&mut self, clothing_piece: Clothing) {
        self.clothing_storage.push(clothing_piece);
    }
    
    pub fn display(&self) -> String {
        format!(
            "
            +--------------------------------------+
                Population {pop}          
            +--------------------------------------+
                Supplies: {supplies}
            +--------------------------------------+
                Attire: 
            +--------------------------------------+
            ",
            pop = self.population,
            supplies = self.supplies.display(),
        )
    }
}

#[derive(Debug)]
pub struct Supplies {
    wood: u32,
    food: u32,
    water: u32,
}

impl Supplies {
    pub fn new(amount: &str) -> Supplies {
        if amount == "low" {
            Supplies {
                wood: 40,
                food: 20,
                water: 20,
            }
        }
        else if amount == "medium" {
            Supplies {
                wood: 50,
                food: 25,
                water: 25,
            }
        }
        else {
            Supplies {
                wood: 60,
                food: 30,
                water: 30,
            }
        }
    }

    pub fn display(&self) -> String {
        format!("
            wood: {}
            food: {}
            water: {}",
            self.wood,
            self.food,
            self.water,
        )
    }

    pub fn reduce_all_by(&mut self, amount: u8) {
        let amount_u32 = amount as u32;
        self.wood -= amount_u32;
        self.food -= amount_u32;
        self.water -= amount_u32;
    }
}

#[cfg(test)]
mod test_caravan {
    use crate::structs::material;

    use super::*;
    

    #[test]
    fn test_new() {
        let new_caravan = Caravan::new();

        assert_eq!(new_caravan.population, 8);

        // ! Look into how to test for same type
        // assert_type_eq!(new_caravan.clothing_storage, Vec<Clothing>);
    }

    #[test]
    fn test_store_clothing() {
        let clothing: Clothing = Clothing::new("cotton shirt", WearLocation::Chest, Material::Cotton);
        let mut caravan: Caravan = Caravan::new();
        caravan.store_clothing(clothing);

        assert_eq!(caravan.clothing_storage.len(), 1);
    }
}

struct StatusEffect {
}

struct PopulationStats {
    number: u8,
    status_effects: Vec<StatusEffect>,
    hunger: u8,
    dehydration: u8,
    sick: u8,
    injured: u8,
    wood_production: i8,
    food_production: i8,
    water_production: i8,
}

trait Stat {
    fn get_stat(&self) -> ();
}

// impl Stat for PopulationStats {
//     fn get_stat(&self, target: &str)
// }
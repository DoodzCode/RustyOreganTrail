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
                Supplies: 
            +--------------------------------------+
                Attire: 
            +--------------------------------------+
            ",
            pop = self.population
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
                wood: 10,
                food: 15,
                water: 20,
            }
        }
        else if amount == "medium" {
            Supplies {
                wood: 15,
                food: 20,
                water: 25,
            }
        }
        else {
            Supplies {
                wood: 20,
                food: 25,
                water: 30,
            }
        }
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
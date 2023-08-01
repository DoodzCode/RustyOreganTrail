use super::{
    clothing::{Attire, Clothing, WearLocation},
    material::Material,
};

pub struct Caravan {
    population: u8,
    supplies: Supplies,
    population_attire: Attire,
    clothing_storage: Vec<Clothing>,
}

impl Caravan {
    pub fn new() -> Caravan {
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
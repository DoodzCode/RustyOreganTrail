
pub struct Caravan {
    population: u8,
    supplies: Supplies,
    
}

impl Caravan {
    pub fn new() -> Caravan {
        Caravan {
            population: 8, 
            supplies: Supplies::new("low"),
        }
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
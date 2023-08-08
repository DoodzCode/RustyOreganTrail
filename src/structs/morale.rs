struct Morale {
    level: MoraleLevel,
    value: i8,
}

impl Morale {
    fn calculate_value(&self) -> i8 {}

    fn calculate_level(&self) -> MoraleLevel {
        let value = self.value;
    }
}

enum MoraleLevel {
    Min,
    ReallyLow,
    Low,
    Normal,
    High,
    ReallyHigh,
    Max,
}

impl MoraleLevel {
    fn get_value(&self) -> i8 {
        match self {
            MoraleLevel::Min => -100,
            MoraleLevel::ReallyLow => -70,
            MoraleLevel::Low => -30,
            MoraleLevel::Normal => 0,
            MoraleLevel::High => 30,
            MoraleLevel::ReallyHigh => 50,
            MoraleLevel::Max => 100,
        }
    }
}

#[derive(Debug)]
pub struct GameData {
    trust_level: u8,
    population: u8,
    injured_population: u8,
    sick_population: u8,
    morale: i8,
    cold_resist: u8,
    heat_resist: u8,
    wagon_durability: u8,
    food_stock: u8,
    wood_stock: u8,
    water_stock: u8,
    axes_in_inventory: u8,
    knives_in_inventory: u8,
    hammers_in_inventory: u8,
    location: u8,
    miles_travelled: u32,
    days_travelled: u8,
}

// fn get_morale_level(morale_value: i8) -> MoraleLevel {
//     match morale_value {
       
//     }
// }

#[cfg(test)]
mod test_morale {
    use super::*;

    #[test]
    fn test_idea() {
        let gd = GameData {
            trust_level: 50,
            population: 20,
            injured_population: 1,
            sick_population: 2,
            morale: 0,
            cold_resist: 20,
            heat_resist: 40,
            wagon_durability: 100,
            food_stock: 15,
            wood_stock: 15,
            water_stock: 15,
            location: 1,
            axes_in_inventory: 5,
            knives_in_inventory: 5,
            hammers_in_inventory: 2,
            miles_travelled: 0,
            days_travelled: 0,
        };

        println!("Current Morale: {}",)
    }
}

enum Material {
    Fur,
    Wool,
    Leather,
    Cotton,
}

trait HasTempResistance {
    fn get_cold_resistance(&self) -> u8;
    fn get_heat_resistance(&self) -> u8;
}

impl HasTempResistance for Material {
    fn get_cold_resistance(&self) -> u8 {
        match &self {
            Material::Fur => 10,
            Material::Wool => 6,
            Material::Leather => 3,
            Material::Cotton => 0,
        }
    }
    fn get_heat_resistance(&self) -> u8 {
        match &self {
            Material::Fur => 0,
            Material::Wool => 3,
            Material::Leather => 6,
            Material::Cotton => 10,
        }
    }
}

enum Temp {
    Freezing,    // 0
    Cold,        // 0
    Chilly,      // 0
    Comfortable, // 0
    Warm,        // 30
    Hot,         // 60
    Scorching,   // 100
}

trait HasTempRating {
    fn get_heat_rating(&self) -> u8;

    fn get_cold_rating(&self) -> u8;
}

impl HasTempRating for Temp {
    fn get_cold_rating(&self) -> u8 {
        match &self {
            Temp::Freezing => 100,
            Temp::Cold => 60,
            Temp::Chilly => 30,
            Temp::Comfortable => 0,
            Temp::Warm => 0,
            Temp::Hot => 0,
            Temp::Scorching => 0,
        }
    }

    fn get_heat_rating(&self) -> u8 {
        match &self {
            Temp::Freezing => 0,
            Temp::Cold => 0,
            Temp::Chilly => 0,
            Temp::Comfortable => 0,
            Temp::Warm => 30,
            Temp::Hot => 60,
            Temp::Scorching => 100,
        }
    }
}

fn check_temp(temp: &Temp) {}
#[cfg(test)]
mod clothing_tests {

    use super::*;

    #[test]
    fn test_temp() {
        let t_temp = Temp::Freezing;

        assert_eq!(t_temp.get_cold_rating(), 100);
        assert_eq!(t_temp.get_heat_rating(), 0);

        println!("cold rating: {}", t_temp.get_cold_rating());
        println!("heat rating: {}", t_temp.get_heat_rating());
    }
    #[test]
    fn test_resistance() {
        let material = Material::Fur;

        assert_eq!(material.get_cold_resistance(), 10);
        assert_eq!(material.get_heat_resistance(), 0);

        println!("cold resistance: {}", material.get_cold_resistance());
        println!("heat resistance: {}", material.get_heat_resistance());
    }
}

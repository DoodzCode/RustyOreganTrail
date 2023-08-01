use super::{
    material::Material,
    traits::{TempRating, TempResistance},
    weather::Temp,
};



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
        let material: Material = Material::Fur;

        assert_eq!(material.get_cold_resistance(), 10);
        assert_eq!(material.get_heat_resistance(), 0);

        println!("cold resistance: {}", material.get_cold_resistance());
        println!("heat resistance: {}", material.get_heat_resistance());
    }
}

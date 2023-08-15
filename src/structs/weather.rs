use super::traits::{Description, TempRating};
use uuid::Uuid;

#[derive(Debug)]
pub enum Weather {
    Clear,
    Rainy,
    Cloudy,
    Foggy,
}

impl Weather {
    fn travel_cost_modifier(&self) -> i8 { 
        match self {
            Weather::Clear => 0,
            Weather::Rainy => 2,        // ! subject to change
            Weather::Cloudy => 1,
            Weather::Foggy => 2,
        }
    }
}

impl Description for Weather {
    fn get_description(&self) -> String {
        match self {
            Weather::Clear => "The skies are clear and you can see ".to_string(),
            Weather::Rainy => {
                "Rain falls heavily and obscures your vision, however you can make out ".to_string()
            }
            Weather::Cloudy => {
                "The land is darkened by clouds, but in the distance you can see ".to_string()
            }
            Weather::Foggy => {
                "Dense fog obscures your vision, you aren't able to see anything.".to_string()
            }
        }
    }
}

pub enum Temp {
    Freezing,    // 0
    Cold,        // 0
    Chilly,      // 0
    Comfortable, // 0
    Warm,        // 30
    Hot,         // 60
    Scorching,   // 100
}

impl TempRating for Temp {
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

#[cfg(test)]
mod test_weather {
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
    fn test_travel_cost_modifier() {
        let t_clear: Weather = Weather::Clear;
        let t_rainy: Weather = Weather::Rainy;
        let t_cloudy: Weather = Weather::Cloudy;
        let t_foggy: Weather = Weather::Foggy;

        assert_eq!(t_clear.travel_cost_modifier(), 0);
        assert_eq!(t_rainy.travel_cost_modifier(), 2);
        assert_eq!(t_cloudy.travel_cost_modifier(), 1);
        assert_eq!(t_foggy.travel_cost_modifier(), 2);
    }
}

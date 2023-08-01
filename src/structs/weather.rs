use super::traits::{Description, TempRating};
use uuid::Uuid;

#[derive(Debug)]
pub enum Weather {
    Clear,
    Rainy,
    Cloudy,
    Foggy,
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
mod test {
    use super::*;

    #[test]
    fn test_Weather() {}
}

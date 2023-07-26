use uuid::Uuid;

pub struct Weather {
    id: Uuid,
    temp: i8,
    precipitation: u8,
    humidity: u8,
    wind_speed: u8,
    biome_id: Uuid,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Weather() {
        let t_weather = Weather {
            id: Uuid::new_v4(),
            temp: i8::from(45),
            precipitation: u8::from(32),
            humidity: u8::from(62),
            wind_speed: u8::from(12),
            biome_id: Uuid::new_v4(),
        };
        assert_eq!(t_weather.temp, 45);
        assert_eq!(t_weather.precipitation, 32);
        assert_eq!(t_weather.humidity, 62);
        assert_eq!(t_weather.wind_speed, 12);
    }
}
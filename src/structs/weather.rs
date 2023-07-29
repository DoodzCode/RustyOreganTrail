use uuid::Uuid;

pub struct Weather<'a> {
    id: Uuid,
    temp: i8,
    precipitation: u8,
    humidity: u8,
    wind_speed: u8,
    biome_id: &'a Uuid,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Weather() {
        let b_id: Uuid = Uuid::new_v4();
        let t_weather: Weather<'_> = Weather {
            id: Uuid::new_v4(),
            temp: i8::from(45),
            precipitation: u8::from(32),
            humidity: u8::from(62),
            wind_speed: u8::from(12),
            biome_id: &b_id,
        };
        assert_eq!(t_weather.temp, 45);
        assert_eq!(t_weather.precipitation, 32);
        assert_eq!(t_weather.humidity, 62);
        assert_eq!(t_weather.wind_speed, 12);
        assert_eq!(t_weather.biome_id, &b_id);
    }
}
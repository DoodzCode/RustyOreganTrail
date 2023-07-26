use Uuid::new_V4;

pub struct Weather {
    id: Uuid,
    temp: i8,
    precipitation: u8,
    humidiity: u8,
    wind_speed: u8,
    biome_id: Uuid,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Weather() {

    }
}
use Uuid::new_V4;

pub struct Point{
    id: Uuid,
    zone_id: Uuid,
    biome_id: Uuid,
    weather_id: Uuid,
    description: String,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Point() {
        let zone_id: new_V4();
        let biome_id: new_V4();
        let weather_id: new_V4();
        let t_point = Point {
            id: new_V4(),
            zone_id = &zone_id,
            biome_id: &biome_id,
            weather_id: &weather_id,
            description: String::new(),
        };
        assert_eq!(t_point.zone_id, zone_id);
    }
}
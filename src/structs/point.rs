use uuid::Uuid;

#[derive(Debug)]
pub struct Point<'a> {
    id: Uuid,
    zone_id: &'a Uuid,
    biome_id: &'a Uuid,
    weather_id: &'a Uuid,
    description: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Point() {
        let zone_id = Uuid::new_v4();
        let biome_id = Uuid::new_v4();
        let weather_id = Uuid::new_v4();

        let t_point: Point = Point {
            id: Uuid::new_v4(),
            zone_id: &zone_id,
            biome_id: &biome_id,
            weather_id: &weather_id,
            description: "Thing?".to_string(),
        };

        assert_eq!(t_point.zone_id, &zone_id);
        assert_eq!(t_point.biome_id, &biome_id);
        assert_eq!(t_point.weather_id, &weather_id);
        assert_eq!(t_point.description, "Thing?".to_string());

        dbg!(t_point);
    }
}

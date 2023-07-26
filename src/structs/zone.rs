use uuid::Uuid;

#[derive(Debug)]
pub struct Zone {
    id: Uuid,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Zone() {
        let t_zone: Zone = Zone {
            id: Uuid::new_v4(),
            name: "Test Zone".to_string(),
        };

        assert_eq!(t_zone.name, "Test Zone".to_string());

        dbg!(t_zone);
    }
}
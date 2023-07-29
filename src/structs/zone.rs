use uuid::Uuid;

#[derive(Debug)]
pub struct Zone {
    id: Uuid,
    name: String,
}

impl Zone {
    pub fn new(name: &str) -> Zone {
        Zone {
            id: Uuid::new_v4(),
            name: String::from(name),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Zone() {
        let t_zone: Zone = Zone::new("Test Zone");
        assert_eq!(t_zone.name, "Test Zone".to_string());
        dbg!(t_zone);
    }
}
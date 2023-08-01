use uuid::Uuid;

#[derive(Debug)]
pub struct Biome {
    id: Uuid,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Biome() {
        let t_biome: Biome = Biome {
            id: Uuid::new_v4(),
            name: "Test Biome".to_string(),
        };

        assert_eq!(t_biome.name, "Test Biome".to_string());

        dbg!(t_biome);
    }
}

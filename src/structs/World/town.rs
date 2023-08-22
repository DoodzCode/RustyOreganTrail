use uuid::Uuid;

pub struct Town {
    id: Uuid,
    name: String,
    population: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Town() {
        let pop = 200;
        let t_town = Town {
            id: Uuid::new_v4(),
            name: String::from("concord"),
            population: pop,
        };
        assert_eq!(t_town.name, String::from("concord"));
        assert_eq!(t_town.population, 200);
    }
}

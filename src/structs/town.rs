use Uuid::new_V4;

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

    }
}
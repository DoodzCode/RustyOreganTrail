use Uuid::new_V4;

pub struct Biome {
    id: Uuid,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Biome() {

    }
}
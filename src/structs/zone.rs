use uuid::Uuid;

pub struct Zone {
    id: Uuid,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Zone() {

    }
}
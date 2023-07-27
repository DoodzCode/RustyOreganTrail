use uuid::Uuid;

#[derive(Debug)]
// pub struct Zone {
//     id: Uuid,
//     name: String,
// }

// impl Zone {
//     pub fn new(name: &str) -> Zone {
//         Zone {
//             id: Uuid::new_v4(),
//             name: String::from(name),
//         }
//     }
// }
//! Got Biomes and Zones mixed up. Will fix later.
enum Zone {
    Plains,
    Hills,
    Desert,
    Forest,
    Mountains,
}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Zone() {
        let z_plains: Zone = Zone::Plains;
        let z_hills: Zone = Zone::Hills;
        let z_desert: Zone = Zone::Desert;
        let z_forest: Zone = Zone::Forest;
        let z_mountains: Zone = Zone::Mountains;

        // assert_eq!(t_zone.name, "Test Zone".to_string());

        dbg!();
    }
}
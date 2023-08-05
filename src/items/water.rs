
#[derive(Debug)]
enum MedicalItem {
    Bandage(ItemStats),
    Tonic(ItemStats),
}

impl MedicalItem {
    pub fn new(item_type: &str) -> MedicalItem {
        if item_type == "bandage" {
            MedicalItem::Bandage(ItemStats::new("bandage", "a bandage", 1))
        } else {
            MedicalItem::Tonic(ItemStats::new("tonic", "a tonic", 1))
        }
    }
}




#[derive(Debug)]
struct Capacity {
    pub max_capacity: u32,
    pub used_capacity: u32,
}

#[derive(Debug)]
enum WaterContainer {
    Waterskin(ItemStats, Capacity),
    WaterBarrel(ItemStats, Capacity),
}

impl WaterContainer {
    pub fn new(item_type: &str) -> WaterContainer {
        if item_type == "barrel" {
            WaterContainer::WaterBarrel(
                ItemStats::new("water barrel", "a water barrel", 1),
                Capacity {
                    max_capacity: 10,
                    used_capacity: 10,
                },
            )
        } else {
            WaterContainer::Waterskin(
                ItemStats::new("waterskin", "a waterskin", 1),
                Capacity {
                    max_capacity: 2,
                    used_capacity: 2,
                },
            )
        }
    }
}

// impl Container for WaterContainer {

// }

trait Container {
    fn get_max_capacity(&self) -> u32;
    fn get_used_capacity(&self) -> u32;
}

// WaterContainer::Barrel

#[derive(Debug)]
struct ItemStats {
    name: String,
    description: String,
    quantity: u32,
}

impl ItemStats {
    pub fn new(name: &str, description: &str, quantity: u32) -> ItemStats {
        ItemStats {
            name: String::from(name),
            description: String::from(description),
            quantity: quantity,
        }
    }
}

impl Capacity {
    pub fn new(max: u32, current: u32) -> Capacity {
        Capacity {
            max_capacity: max,
            used_capacity: current,
        }
    }
}

#[cfg(test)]
mod test_water_containers {
    use super::*;

    #[test]
    fn test_stuff() {
        let barrel = WaterContainer::new("barrel");
        let waterskin = WaterContainer::new("waterskin");
        let bandage = MedicalItem::new("bandage");
        // let bandage = Item::create(MedicalItem::Bandage);
        // let axe = Itemm::create(Tool::Axe);

        dbg!(barrel);
        dbg!(waterskin);
        dbg!(bandage);

        // let waterskin =
    }
}

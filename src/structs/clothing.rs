use super::{
    material::Material,
    traits::{TempRating, TempResistance},
    weather::Temp,
};

/*
    TOOLS
        tools -> resource
        obtain wood
        obtain food
        herbel medicine?
        COOKING
            pot
            cast_iron_pan
        HUNTING
            rabbits
            buffolo
        FISHING
            fish
 */

#[derive(Debug)]
pub struct Attire {
    pub head: Clothing,
    pub chest: Clothing,
    pub legs: Clothing,
    pub feet: Clothing,
    pub hands: Clothing, 
}

impl Attire {
    fn calc_cold_resistance(&self) ->u8 {
        let mut total = 0;
        total += self.head.get_cold_resistance();
        total += self.chest.get_cold_resistance();
        total += self.legs.get_cold_resistance();
        total += self.feet.get_cold_resistance();
        total += self.hands.get_cold_resistance();
        total
    }

    pub fn equip_head(&mut self, new_clothing_piece: Clothing) {
        //! Eventually need to think about retaining the piece that is being replaced
        self.head = new_clothing_piece;
    }

    pub fn equip_chest(&mut self, new_chest_piece: Clothing) {
        self.chest = new_chest_piece;
    }

    pub fn equip_legs(&mut self, new_pants: Clothing) {
        self.legs = new_pants;
    }

    pub fn equip_feet(&mut self, new_shoes: Clothing) {
        self.feet = new_shoes;
    }

    pub fn equip_hands(&mut self, new_gloves: Clothing) {
        self.hands = new_gloves
    }
    // Function that returns a formatted string showing pieces of clothing worn
    /*
        Current Attire:
        Head: Fur Hat,
        Chest: Cotton Shirt,
        Legs: Leather Pants,
        Feet: Leather Shoes,
        Hands: Nothing
     */

    fn get_display(&self) -> String {
        
        let head: &String = &self.head.description;
        let chest: &String = &self.chest.description;
        let legs: &String = &self.legs.description;
        let feet: &String = &self.feet.description;
        let hands: &String = &self.feet.description;
        format!(
            "
            Head: {phead}
            Chest: {pchest}
            Legs: {plegs}
            Feet: {pfeet}
            Hands: {phands}
            ",
            phead = head, pchest = chest, plegs = legs, pfeet = feet, phands = hands
            )
    }
    
}

#[derive(Debug)]
pub struct Clothing {
    description: String,
    wear_location: WearLocation,
    material: Material,
}


impl Clothing {
    fn get_heat_resistance(&self) -> u8 {
        let heat_base: u8 = self.material.get_heat_resistance();
        let bonus: u8 = self.wear_location.get_resistance_bonus();
        heat_base + bonus
    }

    fn get_cold_resistance(&self) -> u8 {
        let cold_base: u8 = self.material.get_cold_resistance();
        let bonus: u8 = self.wear_location.get_resistance_bonus();
        cold_base + bonus
    }

    pub fn new(description: &str, wear_location: WearLocation, material: Material) -> Clothing {
        Clothing {
            description: String::from(description),
            wear_location: wear_location,
            material: material,
        }
    }

    pub fn blank() -> Clothing {
        Clothing::new("Nothing", WearLocation::Any, Material::None)
    }
}

#[derive(Debug)]
pub enum WearLocation {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    Any,
}


trait ResistanceBonus {
    fn get_resistance_bonus(&self) -> u8;
}


impl ResistanceBonus for WearLocation {
    fn get_resistance_bonus(&self) -> u8 {
        match &self {
            WearLocation::Head => 10,
            WearLocation::Chest => 15,
            WearLocation::Legs => 15,
            WearLocation::Feet => 5,
            WearLocation::Hands => 5,
            WearLocation::Any => 0,
        }
    }
}


#[cfg(test)]
mod test_clothing {
    use super::*;

    #[test]
    fn test_clothing_cold_resistance() {
        let fur_hat: Clothing = Clothing {
            description: "Fur Hat".to_string(),
            wear_location: WearLocation::Head,
            material:Material::Fur
        };

        let wool_hat: Clothing  = Clothing {
            description: "Wool Hat".to_string(),
            wear_location: WearLocation::Head,
            material: Material::Wool,
        };
        let fur_hat_resistance: u8 = fur_hat.get_cold_resistance();
        let wool_hat_resistance:u8 = wool_hat.get_cold_resistance();
        
        assert_ne!(fur_hat_resistance, wool_hat_resistance);
        assert!( 
            fur_hat_resistance > wool_hat_resistance,
            "fur:{} wool:{}", 
            fur_hat_resistance, wool_hat_resistance
        );
    }

    #[test]
    fn test_clothing_heat_resistance() {
        let fur_hat: Clothing = Clothing {
            description: "Fur Hat".to_string(),
            wear_location: WearLocation::Head,
            material:Material::Fur
        };

        let wool_hat: Clothing  = Clothing {
            description: "Wool Hat".to_string(),
            wear_location: WearLocation::Head,
            material: Material::Wool,
        };
        let fur_hat_resistance: u8 = fur_hat.get_heat_resistance();
        let wool_hat_resistance:u8 = wool_hat.get_heat_resistance();
        
        assert_ne!(fur_hat_resistance, wool_hat_resistance);
        assert!( 
            fur_hat_resistance < wool_hat_resistance,
            "fur:{} wool:{}", 
            fur_hat_resistance, wool_hat_resistance
        );
    }

    #[test]
    fn test_attire_cold_resistance() {
        let fur_attire = Attire {
            head: Clothing::new("Fur Hat", WearLocation::Head, Material::Fur),
            chest: Clothing::new("Fur Shirt", WearLocation::Chest, Material::Fur),
            legs: Clothing::new("Fur Pants", WearLocation::Legs, Material::Fur),
            feet: Clothing::new("Fur Slippers", WearLocation::Feet, Material::Fur),
            hands: Clothing::new("Fur Gloves", WearLocation::Hands, Material::Fur),
        };
        let cotton_attire = Attire {
            head: Clothing::new("Fur Hat", WearLocation::Head, Material::Cotton),
            chest: Clothing::new("Fur Shirt", WearLocation::Chest, Material::Cotton),
            legs: Clothing::new("Fur Pants", WearLocation::Legs, Material::Cotton),
            feet: Clothing::new("Fur Slippers", WearLocation::Feet, Material::Cotton),
            hands: Clothing::new("Fur Gloves", WearLocation::Hands, Material::Cotton),
        };

        let mixed_attire = Attire {
            head: Clothing::new("Fur Hat", WearLocation::Head, Material::Fur),
            chest: Clothing::new("Fur Shirt", WearLocation::Chest, Material::Wool),
            legs: Clothing::new("Fur Pants", WearLocation::Legs, Material::Leather),
            feet: Clothing::new("Fur Slippers", WearLocation::Feet, Material::Leather),
            hands: Clothing::new("Fur Gloves", WearLocation::Hands, Material::Cotton),
        };


        let fur_cold_resist: u8 = fur_attire.calc_cold_resistance();
        let cotton_cold_resist: u8 = cotton_attire.calc_cold_resistance();
        let mixed_cold_resist: u8 = mixed_attire.calc_cold_resistance();

        dbg!(fur_cold_resist);
        dbg!(cotton_cold_resist);
        dbg!(mixed_cold_resist);


        assert!(
            fur_cold_resist > cotton_cold_resist,
            " fur:{} cotton:{}",
            fur_cold_resist, cotton_cold_resist
        );

    }
}
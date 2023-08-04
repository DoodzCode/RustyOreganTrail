use crate::structs::{caravan::Caravan, traits::Item};

// * import Item from structs::traits::Item and implement the trait onto any struct that represents a physical item
// * since the trait's methods require a name, description, and quantity, the struct should also have those fields


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// An example of one way these items could be set up

/// Represents ailments that are not physical: Sickness, Disease, Fever
enum InternalAilment {
    Cold,
    Flue,
    Pneumonia,
    Dysentery,
    Typhoid,
}

/// Represents physical injuries and ailments: Sprained Ankle, Concussion, 
enum PhysicalAilment {
    MinorInjury,
    ModerateInjury,
    SeverInjury,
    Concussion,

}



enum ItemType {
    Consumable,
    Useable,
    Tool,
    Clothing,
}

pub struct Tonic {
    name: String,
    description: String,
    item_type: ItemType,
    quantity: u32,
}



impl Item for Tonic {
    fn get_quantity(&self) -> u32 {
        self.quantity
    }

    fn get_description(&self) -> String {
        String::from(self.description)
    }

    fn get_name(&self) -> String {
        String::from(self.name)
    }

    fn get_type(&self) -> Self {
        self.get_type()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub enum InjuryType {
    Cold,
    Fever,
    Pneumonia,
}

pub enum OddInjury {
    Dysentery,
    Hypothermia,                        // ! RENAME ALL ENUMS 
    Heatstroke,                             // LOL
}


pub enum Injury {
    MinorInjury,
    ModerateInjury,
    SeriousInjury,
}

pub struct Bandage {
    bandage: u32, // ? what does this field represent?
    name: String,
    // type: BandageType
    target: Injury,
    action: String,
}

// ! how struct "bandage" is applied shall effect enum "injury" differeantly
impl Bandage {
    fn address_injury(&self) {
        let gauze = Bandage::new(&self);
        let suture = Bandage::new(&self);
        if gauze.name == "gauze" {
            if gauze.target = Injury::MinorInjury {
                gauze.action = "remove";
            }
            else if gauze.target = Injury::ModerateInjury {
                gauze.action = "reduce-duration";
            }
            else if gauze.target = Injury::SeriousInjury {
                gauze.action = "reduce-duration";
            }
            else {"cannot apply to this injury type"};
        }
        if suture.name == "suture" {
            if suture.target = Injury::MinorInjury {
                suture.action = "cannot apply to this injury type"
                }
            }
            else if suture.target = Injury::ModerateInjury {
                suture.action = "reduce-duration"
            }
            else if suture.target = Injury::SeriousInjury {
                suture.action = "pause-duration"
            }
            else {"cannot be applied to this injury type"}
        }
    }


// you've got a few implementations of Bandage around here, considering putting all fuctions within
// a single impl block.
impl new for Bandage {
    fn new(&self) ->self {
        let bandage = Bandage{
            bandage: u32::new(bandage), 
            name: String::from(name),
            // name: String::from(name),
            target: Injury,
            action: String::from(action),
        };
    }
}
// pub struct Bandage {
//     bandage: u32,
//     name: String,
//     applies_to: InjuryType,
// }

// impl Bandage {
    // fn address_minor_injury(&self) {
    //     let status_effect_target = InjuryStatusEffect::MinorInjury;
    //     let action: String;
    //     let bandage = Bandage::new();
    //     if bandage.name == "bandage" {
    //         action = "remove";
    //     }
    // }
// }
pub struct Medicine {
    medicine: u32,
    name: String,
    applies_to: InjuryType,
}

impl Medicine {
    fn address_ailment(&self)
}

pub trait new {
    fn new(&self) -> self;
}

// Vec<StatusEffects> = [
//     {name: String, mop: StatMod, duration: 2, InjuryType::Fever},
//     // {name: String, mop: StatMod, duration: 2, InjuryType::MinorInjury},
// ]

// let bandage = Bandage { name: "A simple wrap", applies_to: InjuryStatusEffect::MinorInjury}

impl Bandage {
    pub fn apply(&self) -> InjuryType {
        self.applies_to
    }

    pub fn injury(&self, caravan: Caravan) {
        if caravan.population = InjuryStatusEffect::MinorInjury {
            if InjuryStatusEffect::MinorInjury = bandage {
                caravan.population != InjuryStatusEffect::MinorInjury; // ! temporary. doesn't seem logical
            }
        }
        else if caravan.population = InjuryStatusEffect::ModerateInjury {
            if InjuryStatusEffect::ModerateInjury = bandage {
                // ! I have no idea yet
            }
        }
    }
}
pub struct Antibiotics {
    antibiotics: u32,
} 

pub struct Sutures {
    sutures: u32,
}


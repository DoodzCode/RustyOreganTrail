use crate::structs::{caravan::Caravan, trailpoint::TrailPoint, traits::Name};

pub enum InjuryType {
    Cold,
    Fever,
    Pneumonia,
}

pub enum OddInjury {
    Dysentery,
    Hypothermia,                        // ! RENAME ALL ENUMS
    Heatstroke,
}

pub enum Injury {
    MinorInjury,
    ModerateInjury,
    SeriousInjury,
}

pub struct Bandage {
    bandage: u32,
    name: String,
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

impl new for Bandage {
    fn new(&self) ->self {
        let bandage = Bandage{
            bandage: u32::new(bandage),
            name: String::from(name),
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


use crate::structs::{
    trailpoint::{TrailPoint, _generate_tiny_trail},
    terrain::Terrain,
};

use crate::commands::game_commands::{_generate_map, add_trail_to_map, build_forest};


/// Holds all 'global' data for the game
#[derive(Debug)]
pub struct GameData {
    /// Struct representing a wagon which holds resources and has durability
    pub wagon: Wagon,
    /// Struct that represents the people
    pub people: People,
    /// Struct used to hold any "rates" at which stats are modified: Needs a better name
    pub gather_rates: GatherRates,

    pub trail: Vec<TrailPoint>,
    // pub trail_iterator: Option<std::vec::IntoIter<TrailPoint>>, // TODO Make this field private
    pub current_location: Option<TrailPoint>,
    pub map: Vec<Vec<Terrain>>,
    
    // Needs refactoring
    pub cold_resist: u8,
    pub heat_resist: u8,
    /// Represents the caravan population's trust in the player as a leader
    pub trust_level: u8,// ! [PATCH]  Deprecated

    pub axes_in_inventory: u8,
    pub knives_in_inventory: u8,
    pub hammers_in_inventory: u8,

    // location: (u8, u8), Deprecated: The current location is now tracked through the TrailPoints
    /// How many miles the caravan has travelled thus far
    pub miles_travelled: u32,
    /// Number of miles travelled in current day
    pub miles_today: u32,
    /// How many game days the player has been playing
    pub days_travelled: u8,
    /// How many hours are left in the current game day
    pub daylight_hours: u8,
    /// The current TrailPoint along the Trail
    pub current_position: usize,
}

impl GameData {
    pub fn new_test_data() -> GameData {

        let mut gd = GameData {
            trail: _generate_tiny_trail(),
            map: _generate_map(100, 100),
            // trail_iterator: None,
            current_location: None,
            people: People {
                population: 20,
                injured_population: 1,
                sick_population: 2,
                morale: 75,
                hungry: false,
                starving: false,
                thirsty: false,
                dehydrated: false,
            },
            gather_rates: GatherRates {
                food: 5,
                water: 5,
                wood: 5,
                repair: 5,
                morale: 5,
            },
            trust_level: 50,
            cold_resist: 20,
            heat_resist: 40,
            wagon: Wagon {
                durability: 100,
                food_stock: 15,
                wood_stock: 15,
                water_stock: 15,
            },
            axes_in_inventory: 5,
            knives_in_inventory: 5,
            hammers_in_inventory: 2,
            // location: (50, 50),
            miles_travelled: 0,
            miles_today: 0,
            days_travelled: 0,
            daylight_hours: 12,
            current_position: 0,
        };

        gd.mod_map();
        // gd.build_trail_iterator();
        gd
    }

    fn mod_map(&mut self) {
        build_forest((55, 45), &mut self.map, 4);
        build_forest((40, 55), &mut self.map, 3);
        build_forest((35, 50), &mut self.map, 2);
        add_trail_to_map(&mut self.map);
    }

    pub fn current_location(&self) -> &TrailPoint {
        &self.trail[self.current_position]
    }

    // fn build_trail_iterator(&mut self) {
    //     self.trail_iterator = Some(self.trail.into_iter());
    //     self.current_location = Some(self.trail_iterator.unwrap().next().unwrap())
    //     // TODO unwrap circumvents error catching
    // }

    // pub fn get_next_location(&mut self) {
    //     let t_iter: std::vec::IntoIter<&TrailPoint>;
    //     match &mut self.trail_iterator {
    //         Some(t_iter) => {
    //             self.current_location = Some(t_iter.next().unwrap());
    //         },  
    //         None => ()
    //     }
    // }
}

#[derive(Debug)]
pub struct Wagon {
    pub durability: u8,
    pub food_stock: u8,
    pub wood_stock: u8,
    pub water_stock: u8,
}

#[derive(Debug)]
pub struct People {
    pub population: u8,
    pub injured_population: u8,
    pub sick_population: u8,
    pub morale: i8,
    pub hungry: bool,
    pub starving: bool,
    pub thirsty: bool,
    pub dehydrated: bool,
}

// TODO rename this struct: we now have rates other than gathering
#[derive(Debug)]
pub struct GatherRates {
    pub food: u8,
    pub water: u8,
    pub wood: u8,
    pub repair: u8,
    pub morale: i8,
}

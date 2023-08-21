
use crate::structs::World::{
    trailpoint::{TrailPoint, _generate_tiny_trail},
};
use crate::structs::Game::trailmap::TrailMap;
use crate::structs::People::person::{People, GatherRates};
use crate::structs::Objects::wagon::Wagon;

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
    pub map: TrailMap,
    
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

        let mut gd: GameData = GameData {
            trail: _generate_tiny_trail(),
            map:TrailMap::create(100, 100),
            current_location: None,
            people: People::create_test_object(),
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
            miles_travelled: 0,
            miles_today: 0,
            days_travelled: 0,
            daylight_hours: 12,
            current_position: 0,
        };

        gd.mod_map();
        gd
    }

    fn mod_map(&mut self) {
        self.map.build_forest((55, 45), 4);
        self.map.build_forest((40, 55), 3);
        self.map.build_forest((35, 50), 2);
        self.map.add_trail();
    }

    pub fn current_location(&self) -> &TrailPoint {
        &self.trail[self.current_position]
    }
}

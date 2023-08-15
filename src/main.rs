// mod items;
// use rand;
mod commands;
mod common;
mod structs;

use commands::{get_input, match_command, opening_screen};
use common::GameData;
use structs::{
    terrain::Terrain,
    trailpoint::{Coords, TrailPoint, _generate_tiny_trail},
};

fn main() {
    opening_screen();
    let mut game_data = GameData::new_test_data();

    loop {
        let cmd = get_input();
        match_command(cmd, &mut game_data)
    }
}

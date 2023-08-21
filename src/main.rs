// mod items;
// use rand;
mod commands;
mod common;
mod structs;

use commands::game_commands::{get_input, match_command, opening_screen, player_prompt};
use common::GameData;
// use structs::{
//     // terrain::Terrain,
//     trailpoint::{Coords, TrailPoint, _generate_tiny_trail},
// };

fn main() {
    opening_screen();
    let mut game_data: GameData = GameData::new_test_data();
    
    
    loop {
        println!("{:?}", &game_data.people);

        player_prompt(&game_data);
        let cmd = get_input();
        match_command(cmd, &mut game_data)
    }
}

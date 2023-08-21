use colored::Colorize;

use crate::structs::World::{
    trailpoint::{TrailPoint, Coords},
    terrain::Terrain,
};

use crate::common::{
    GameData,
};

use crate::commands::player_commands;


pub fn println_to_player(content: &String) {
    println!("{}", content);
}

/// Matches player input with an available command if any
/// 
/// takes the player's input as a String and the GameData object
pub fn match_command(cmd: String, game_data: &mut GameData) {
    match cmd.as_str() {
        "camp" => player_commands::cmd_camp(game_data),
        "gather" => player_commands::cmd_gather_rate(&game_data.gather_rates),
        "inspect" => player_commands::cmd_inspect(&game_data.wagon),
        "report" => player_commands::cmd_report(&game_data),
        "look" => {
            // TODO this should be a single function somewhere since we are using it multiple times
            println_to_player(&game_data.map.to_string(&game_data.current_location()));
            player_commands::cmd_look(&game_data.trail[game_data.current_position]);
        }
        "peep" => player_commands::cmd_population_report(&game_data.people),
        "survey" => player_commands::cmd_survey(&game_data.current_location()),
        "trust" => player_commands::cmd_inspect_trust_level(&game_data), // ? Is this weird?

        // TODO abstract into a function
        "travel" => {
            player_commands::cmd_travel(
                &mut game_data.current_position,
                &game_data.trail,
                &mut game_data.daylight_hours,
                &mut game_data.miles_travelled,
                &mut game_data.miles_today,
            );
            print_map(&game_data);
            player_commands::cmd_look(&game_data.trail[game_data.current_position]);
        }
        "status" => player_commands::cmd_status(&game_data),
        "map" => print_map(&game_data),
        "dbg" => println!("{:?}", game_data),
        "quit" => player_commands::cmd_quit(),
        "commands" => println!(
            "
            camp    inspect    look    status   travel   quit
        "
        ),
        _ => println!("Command not recognized. Use 'commnds' to see a list of valid commands."),
    }
}

fn print_map(game_data: &GameData ) {
    println_to_player(&game_data.map.to_string(&game_data.current_location()));
}

pub fn opening_screen() {
    println!(
        r"
$$$$$$$\                        $$\                      $$$$$$\                                                          $$$$$$$$\                 $$\ $$\ 
$$  __$$\                       $$ |                    $$  __$$\                                                         \__$$  __|                \__|$$ |
$$ |  $$ |$$\   $$\  $$$$$$$\ $$$$$$\   $$\   $$\       $$ /  $$ | $$$$$$\   $$$$$$\   $$$$$$\   $$$$$$\  $$$$$$$\           $$ | $$$$$$\  $$$$$$\  $$\ $$ |
$$$$$$$  |$$ |  $$ |$$  _____|\_$$  _|  $$ |  $$ |      $$ |  $$ |$$  __$$\ $$  __$$\ $$  __$$\ $$  __$$\ $$  __$$\          $$ |$$  __$$\ \____$$\ $$ |$$ |
$$  __$$< $$ |  $$ |\$$$$$$\    $$ |    $$ |  $$ |      $$ |  $$ |$$ |  \__|$$$$$$$$ |$$ /  $$ |$$ /  $$ |$$ |  $$ |         $$ |$$ |  \__|$$$$$$$ |$$ |$$ |
$$ |  $$ |$$ |  $$ | \____$$\   $$ |$$\ $$ |  $$ |      $$ |  $$ |$$ |      $$   ____|$$ |  $$ |$$ |  $$ |$$ |  $$ |         $$ |$$ |     $$  __$$ |$$ |$$ |
$$ |  $$ |\$$$$$$  |$$$$$$$  |  \$$$$  |\$$$$$$$ |       $$$$$$  |$$ |      \$$$$$$$\ \$$$$$$$ |\$$$$$$  |$$ |  $$ |         $$ |$$ |     \$$$$$$$ |$$ |$$ |
\__|  \__| \______/ \_______/    \____/  \____$$ |       \______/ \__|       \_______| \____$$ | \______/ \__|  \__|         \__|\__|      \_______|\__|\__|
                                        $$\   $$ |                                    $$\   $$ |                                                            
                                        \$$$$$$  |                                    \$$$$$$  |                                                            
                                         \______/                                      \______/
"
    );
    println!("Press Enter to Continue");
    let _ = get_input();
}


pub fn get_input() -> String {
    let mut r_input: String = String::new();
    std::io::stdin().read_line(&mut r_input).unwrap();
    let input: &str = r_input.trim();
    String::from(input).to_lowercase()
}

pub fn player_prompt(game_data: &GameData) {
    // time of day, miles travelled, morale level
    println!(
        "
Morale: questionable
Daylight Hourse Remaining: {daylight}   Miles Travelled Today: {miles}",
        daylight = game_data.daylight_hours,
        miles = game_data.miles_today,
    )
}
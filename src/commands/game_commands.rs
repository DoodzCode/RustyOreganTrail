use colored::Colorize;

use crate::structs::{
    trailpoint::{TrailPoint, Coords},
    terrain::Terrain,
};

use crate::common::{
    GameData,
};

use crate::commands::player_commands;


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
            print_map(&game_data.current_location(), &game_data.map);
            player_commands::cmd_look(&game_data.trail[game_data.current_position]);
        }
        "peep" => player_commands::cmd_population_report(&game_data.people),
        "survey" => player_commands::cmd_survey(&game_data.current_location()),
        "trust" => player_commands::cmd_inspect_trust_level(&game_data), // ? Is this weird?
        "travel" => {
            player_commands::cmd_travel(
                &mut game_data.current_position,
                &game_data.trail,
                &mut game_data.daylight_hours,
                &mut game_data.miles_travelled,
                &mut game_data.miles_today,
            );
            print_map(&game_data.current_location(), &game_data.map);
            player_commands::cmd_look(&game_data.trail[game_data.current_position]);
        }
        "status" => player_commands::cmd_status(&game_data),
        "map" => print_map(&game_data.current_location(), &game_data.map),
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






fn print_map(location: &TrailPoint, map: &Vec<Vec<Terrain>>) {
    let coords: &Coords = &location.coords;
    let radius: u8 = location.weather.visibility();

    let row_start: u8 = coords.0 - radius;
    let col_start: u8 = coords.1 - radius;

    let row_end: u8 = coords.0 + radius;
    let col_end: u8 = coords.1 + radius;

    let player_token = '*';

    for y_coord in row_start..row_end + 1 {
        for x_coord in col_start..col_end + 1 {
            if x_coord == coords.1 && y_coord == coords.0 {
                print!("{} ", player_token.to_string().red().blink());
            } else {
                print_token(&map[usize::from(y_coord)][usize::from(x_coord)]);
            }
        }
        print!("\n");
    }

    fn print_token(terrain: &Terrain) {
        match terrain {
            Terrain::Plains => print!("{} ", terrain.get_token().to_string().bright_yellow()),
            Terrain::Desert => print!("{} ", terrain.get_token()),
            Terrain::Forest => print!("{} ", terrain.get_token()),
            Terrain::Hills => print!("{} ", terrain.get_token()),
            Terrain::Mountain => print!("{} ", terrain.get_token()),
            Terrain::Trail => print!("{} ", terrain.get_token()),
            _ => (),
        }
    }
}

pub fn _generate_map(rows: u8, cols: u8) -> Vec<Vec<Terrain>> {
    let mut map: Vec<Vec<Terrain>> = Vec::new();

    for _ in 0..rows {
        let mut row: Vec<Terrain> = Vec::new();
        for _ in 0..cols {
            row.push(Terrain::Plains);
        }
        map.push(row);
    }

    map
}

/// Generates a square of forest around a single point
pub fn build_forest(coords: (u8, u8), map: &mut Vec<Vec<Terrain>>, radius: u8) {
    let row_start: u8 = coords.0 - radius;
    let col_start: u8 = coords.1 - radius;

    let row_end: u8 = coords.0 + radius;
    let col_end: u8 = coords.1 + radius;

    let mut y: u8 = 0;
    let mut x: u8 = 0;

    for row in map.iter_mut() {
        for point in row.iter_mut() {
            if y >= row_start && y <= row_end {
                if x >= col_start && x <= col_end {
                    if y == row_start || y == row_end || x == col_start || x == col_end {
                        if rand::random() {
                            *point = Terrain::Forest;
                        }
                    } else {
                        *point = Terrain::Forest;
                    }
                }
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
}

pub fn add_trail_to_map(map: &mut Vec<Vec<Terrain>>) {
    let trail_col: u8 = 50;
    let mut y: u8 = 0;
    let mut x: u8 = 0;

    for row in map.iter_mut() {
        for point in row.iter_mut() {
            if x == trail_col {
                *point = Terrain::Trail;
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
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
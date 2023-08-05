mod commands;
mod structs;
mod items;

use commands::{process_command, GameState};
use std::io;
use structs::{
    caravan::Caravan,
    trailpoint::{TrailPoint, _generate_trail},
};

fn print_opening_screen() {
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
}


fn main() {
    let trail: Vec<TrailPoint> = _generate_trail();
    let mut game_state = GameState {
        days: 0,
        miles_travelled: 0,
        travel_hours_left_in_day: 12,
    };
    let mut titer: std::slice::Iter<'_, TrailPoint> = trail.iter();
    let mut caravan = Caravan::new();

    print_opening_screen();
    let _ = get_input();
    // print_motd();
    // let _ = get_input();

    loop {
        match titer.next() {
            Some(location) => loop {
                process_command("look".to_string(), &location, &mut caravan, &mut game_state);
                println!("Enter Command:");
                let input: String = get_input();

                if input == "travel".to_string() {
                    if game_state.ok_to_travel() {
                        println!("You travel onward down the trail...");
                        game_state.reduce_day_hours(2);
                        game_state.increase_miles(2);
                        break;
                    } else {
                        // inform player there is not enough time to travel today
                        println!("The day is coming to an end, you do not have enough time to travel for the day");
                    }
                } else {
                    process_command(input, &location, &mut caravan, &mut game_state);
                }
            },
            None => {
                println!("You have completed the trail!");
                break;
            }
        }
    }
}

fn get_input() -> String {
    let mut r_input: String = String::new();
    io::stdin().read_line(&mut r_input).unwrap();
    let input: &str = r_input.trim();
    String::from(input).to_lowercase()
}

/*

    CAMPING
    at end of day, player will need to camp for the night
    code-wise that means
        the travel hours will be replenished
        the day number will increment




*/

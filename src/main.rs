// mod commands;
// mod items;
mod structs;

// use commands::{process_command, GameState};
use std::io;
use structs::trailpoint::{TrailPoint, _generate_tiny_trail};
// use structs::{
//     caravan::Caravan,
//     trailpoint::{TrailPoint, _generate_trail},
// };

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

#[derive(Debug)]
pub struct GameData {
    trust_level: u8,
    people: People,

    gather_rates: GatherRates,

    cold_resist: u8,
    heat_resist: u8,

    wagon: Wagon,

    axes_in_inventory: u8,
    knives_in_inventory: u8,
    hammers_in_inventory: u8,

    location: u8,
    miles_travelled: u32,
    days_travelled: u8,
}

#[derive(Debug)]
struct Wagon {
    durability: u8,
    food_stock: u8,
    wood_stock: u8,
    water_stock: u8,
}

#[derive(Debug)]
struct People {
    population: u8,
    injured_population: u8,
    sick_population: u8,
    morale: i8,
}

#[derive(Debug)]
struct GatherRates {
    food: u8,
    water: u8,
    wood: u8,
}

fn main() {
    let mut gd = GameData {
        trust_level: 50,
        people: People {
            population: 20,
            injured_population: 1,
            sick_population: 2,
            morale: 0,
        },
        gather_rates: GatherRates {
            food: 5,
            water: 5,
            wood: 5,
        },
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
        location: 1,
        miles_travelled: 0,
        days_travelled: 0,
    };

    print_opening_screen();
    let trail = _generate_tiny_trail();
    let mut t_iter: std::slice::Iter<'_, TrailPoint> = trail.iter();
    let mut current_location = t_iter.next().unwrap();

    loop {
        let player_input = get_input();

        match player_input.as_str() {
            "camp" => cmd_camp(&mut gd),
            "gather" => cmd_gather_rate(&gd.gather_rates),
            "inspect" => cmd_inspect(&gd.wagon),
            "look" => cmd_look(&current_location),
            "peep" => cmd_population_report(&gd.people),
            "survey" => println!(
                "{:?}",
                current_location.terrain.base_resource_availability()
            ),
            "trust" => cmd_inspect_trust_level(&gd), // ? Is this weird?
            "travel" => current_location = t_iter.next().unwrap(),

            "quit" => std::process::exit(0),
            _ => println!("Unknown Command"),
        }
    }

    // fn cmd_travel(current_location: &mut TrailPoint, t_iter: &mut std::slice::Iter<'_, TrailPoint> ) {
    //     current_location = t_iter.next().unwrap();
    // }

    fn cmd_look(current_location: &TrailPoint) {
        println!("{}", current_location.get_description());
    }

    fn cmd_survey(current_location: &TrailPoint) {
        println!(
            "{:?}",
            current_location.terrain.base_resource_availability()
        );
    }

    fn cmd_inspect(wagon: &Wagon) {
        println!("{:?}", wagon);
    }

    fn cmd_population_report(people: &People) {
        println!("{:?}", people);
    }

    fn cmd_inspect_trust_level(game_data: &GameData) {
        println!("{:?}", game_data.trust_level);
    }

    fn cmd_gather_rate(gather_rates: &GatherRates) {
        println!("{:?}", gather_rates);
    }

    /// increases the wagon's food stock by the amount of the food gather rate
    fn gather_food(wagon: &mut Wagon, gather_rates: &GatherRates) {
        wagon.food_stock += gather_rates.food;
    }
    /// increases the wagon's water stock by the amount of the water gather rate
    fn gather_water(wagon: &mut Wagon, gather_rates: &GatherRates) {
        wagon.water_stock += gather_rates.water;
    }
    /// increases the wagon's wood stock by the amount of the wood gather rate
    fn gather_wood(wagon: &mut Wagon, gather_rates: &GatherRates) {
        wagon.wood_stock += gather_rates.wood;
    }

    fn cmd_camp(gd: &mut GameData) {
        println!("You tell the caravan to start breaking down and make camp...");

        let mut workers_left = gd.people.population;
        let mut current_day = gd.days_travelled;
        let mut g_food: u8 = 0;
        let mut g_water: u8 = 0;
        let mut g_wood: u8 = 0;

        println!("How many workers shall be assigned to food?");
        loop {
            let player_input = get_input().parse::<u8>().unwrap();
            if player_input <= workers_left {
                g_food += player_input;
                workers_left -= player_input;
                break;
            } else {
                println!("not enough workers to assign to task");
            }
        }

        println!("How many many workers shall be assigned to water?");
        loop {
            let player_input = get_input().parse::<u8>().unwrap();
            if player_input <= workers_left {
                g_water += player_input;
                workers_left -= player_input;
                break;
            } else {
                println!("not enough workers to be assigned to water?");
            }
        }

        println!("How many many workers shall be assigned to wood?");
        loop {
            let player_input = get_input().parse::<u8>().unwrap();
            if player_input <= workers_left {
                g_wood += player_input;
                workers_left -= player_input;
                break;
            } else {
                println!("not enough workers to be assigned to wood?");
            }
        
        }
        println!(
            "Food: {}, Water: {}, Wood: {}, Remaining: {}",
            g_food, g_water, g_wood, workers_left
        );

        println!("The people head off to their assigned tasks...");

        for _ in 0..g_food {
            gather_food(&mut gd.wagon, &gd.gather_rates);
        }

        for _ in 0..g_water {
            gather_water(&mut gd.wagon, &gd.gather_rates);
        }

        for _ in 0..g_wood {
            gather_wood(&mut gd.wagon, &gd.gather_rates);
        }


        println!("The sun sinks below the horizon and you turn in for the night.");

        // Resource Consumption: Reduce Stocks
        gd.wagon.food_stock -= gd.people.population;
        gd.wagon.water_stock -= gd.people.population;
        gd.wagon.wood_stock -= gd.people.population;

        current_day += 1;
    }
}

// fn main() {
//     let trail: Vec<TrailPoint> = _generate_trail();
//     let mut game_state = GameState {
//         days: 0,
//         miles_travelled: 0,
//         travel_hours_left_in_day: 12,
//     };
//     let gd = GameData {
//         trust_level: 50,
//         population: 20,
//         injured_population: 1,
//         sick_population: 2,
//         morale: 0,
//         cold_resist: 20,
//         heat_resist: 40,
//         wagon_durability: 100,
//         food_stock: 15,
//         wood_stock: 15,
//         water_stock: 15,
//         location: 1,
//         axes_in_inventory: 5,
//         knives_in_inventory: 5,
//         hammers_in_inventory: 2,
//         miles_travelled: 0,
//         days_travelled: 0,
//     };
//     let mut titer: std::slice::Iter<'_, TrailPoint> = trail.iter();
//     let mut caravan = Caravan::new();

//     print_opening_screen();
//     let _ = get_input();
//     // print_motd();
//     // let _ = get_input();

//     loop {
//         match titer.next() {
//             Some(location) => loop {
//                 process_command("look".to_string(), &location, &mut caravan, &mut game_state);
//                 println!("Enter Command:");
//                 let input: String = get_input();

//                 if input == "travel".to_string() {
//                     if game_state.ok_to_travel() {
//                         println!("You travel onward down the trail...");
//                         game_state.reduce_day_hours(2);
//                         game_state.increase_miles(2);
//                         break;
//                     } else {
//                         // inform player there is not enough time to travel today
//                         println!("The day is coming to an end, you do not have enough time to travel for the day");
//                     }
//                 } else {
//                     process_command(input, &location, &mut caravan, &mut game_state);
//                 }
//             },
//             None => {
//                 println!("You have completed the trail!");
//                 break;
//             }
//         }
//     }
// }

fn get_input() -> String {
    let mut r_input: String = String::new();
    io::stdin().read_line(&mut r_input).unwrap();
    let input: &str = r_input.trim();
    String::from(input).to_lowercase()
}

// #[cfg(test)]

/*

    CAMPING
    at end of day, player will need to camp for the night
    code-wise that means
        the travel hours will be replenished
        the day number will increment




*/

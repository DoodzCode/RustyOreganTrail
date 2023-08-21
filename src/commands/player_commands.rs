use crate::structs::trailpoint::TrailPoint;

use crate::commands::game_commands::get_input;
use crate::common::{GameData, GatherRates, People, Wagon};

pub fn cmd_quit() {
    std::process::exit(0);
}

pub fn cmd_travel(
    current_position: &mut usize,
    trail: &Vec<TrailPoint>,
    daylight_hours: &mut u8,
    miles_travelled: &mut u32,
    miles_today: &mut u32,
) {
    // get total travel cost and subtract daylight_hours here
    let travel_cost: u8 = trail[*current_position].travel_cost();

    if *daylight_hours >= travel_cost {
        *daylight_hours -= travel_cost;
        *current_position += 1;
        *miles_travelled += 1;
        *miles_today += 1;
    } else {
        println!("There is not enough daylight left for travelling, you should camp for the night.")
    }
}

pub fn cmd_look(current_location: &TrailPoint) {
    println!("{}", current_location.get_description());
}

pub fn cmd_survey(current_location: &TrailPoint) {
    println!(
        "{:?}",
        current_location.terrain.base_resource_availability()
    );
}

pub fn cmd_inspect(wagon: &Wagon) {
    println!("{:?}", wagon);
}

pub fn cmd_population_report(people: &People) {
    println!("{:?}", people);
}

pub fn cmd_inspect_trust_level(game_data: &GameData) {
    println!("{:?}", game_data.trust_level);
}

pub fn cmd_gather_rate(gather_rates: &GatherRates) {
    println!("{:?}", gather_rates);
}

pub fn cmd_camp(gd: &mut GameData) {
    println!("You tell the caravan to start breaking down and make camp...");

    let mut workers_left = gd.people.population;
    let mut current_day = gd.days_travelled;
    let mut g_food: u8 = 0;
    let mut g_water: u8 = 0;
    let mut g_wood: u8 = 0;
    let mut repairers: u8 = 0;

    let mut entertainment_tonight: bool = false;

    loop {
        println!("what tasks do you want to perform?");
        println!("Options: entertain, tasks");
        let player_input = get_input();
        if player_input == "entertain" {
            entertainment_tonight = true;
            break;
        } else if player_input == "tasks" {
            loop {
                println!("What Tasks do you want to partake in?");
                println!("Options: repair, gather, done");
                let player_input = get_input();

                match player_input.as_str() {
                    "repair" => assign_repairers(&mut workers_left, &mut repairers),
                    "gather" => {
                        assign_gatherers(&mut workers_left, &mut g_food, &mut g_water, &mut g_wood)
                    }
                    "done" => break,
                    _ => (),
                }
            }
            break;
        } else {
            println!("invalid input");
        }
    }

    // run start_new_day()

    // end_of_day()
    if entertainment_tonight == true {
        entertainment(&mut gd.people, &gd.gather_rates);
    } else {
        perform_tasks(&g_food, &g_water, &g_wood, &repairers, gd);
    }
    println!("The sun sinks below the horizon and you turn in for the night.");

    // start_new_day()
    // Resource Consumption: Reduce Stocks
    // Check if there is enough stock for population
    if gd.people.population > gd.wagon.food_stock {
        gd.wagon.food_stock = 0;
        gd.people.hungry = true;
        // TODO make this better
        println!("There is not enough food for everyone, some people will go hungry.");
    } else {
        gd.wagon.food_stock -= gd.people.population;
    }

    if gd.people.population > gd.wagon.water_stock {
        gd.wagon.water_stock = 0;
        gd.people.thirsty = true;
        println!("There is not enough water for everyone, some people will be thirsty.")
    } else {
        gd.wagon.water_stock -= gd.people.population;
    }

    if gd.people.population > gd.wagon.wood_stock {
        gd.wagon.wood_stock = 0;
        // TODO what happens when wood_stock aint got wood?
        println!("Not enough wood for everyone. Something bad might happen...")
    } else {
        gd.wagon.wood_stock -= gd.people.population;
    }

    current_day += 1;
    gd.daylight_hours = 12;
    gd.miles_today = 0;
}

/// Reports data on population, progress, and supplies
pub fn cmd_status(gd: &GameData) {
    println!(
        r"
     ___                                                  ___                               _   
    (  _`\                                               |  _`\                            ( )_ 
    | |_) ) _ __   _      __   _ __   __    ___   ___    | (_) )   __   _ _      _    _ __ | ,_)
    | ,__/'( '__)/'_`\  /'_ `\( '__)/'__`\/',__)/',__)   | ,  /  /'__`\( '_`\  /'_`\ ( '__)| |  
    | |    | |  ( (_) )( (_) || |  (  ___/\__, \\__, \   | |\ \ (  ___/| (_) )( (_) )| |   | |_ 
    (_)    (_)  `\___/'`\__  |(_)  `\____)(____/(____/   (_) (_)`\____)| ,__/'`\___/'(_)   `\__)
                       ( )_) |                                         | |                      
                        \___/'                                         (_)                     
    "
    );

    println!(
        "
Trust Level: {trust}            Date: {date}
Population: {pop}               Days Travelled: {days}
Miles Travelled: {miles}        Daylight Hours Remaining: {daylight}
",
        trust = gd.trust_level,
        pop = gd.people.population,
        date = "11/11/11".to_string(),
        days = gd.days_travelled,
        miles = gd.miles_travelled,
        daylight = gd.daylight_hours,
    )
}

/// increases the wagon's food stock by the amount of the food gather rate
pub fn gather_food(wagon: &mut Wagon, gather_rates: &GatherRates) {
    wagon.food_stock += gather_rates.food;
}
/// increases the wagon's water stock by the amount of the water gather rate
pub fn gather_water(wagon: &mut Wagon, gather_rates: &GatherRates) {
    wagon.water_stock += gather_rates.water;
}
/// increases the wagon's wood stock by the amount of the wood gather rate
pub fn gather_wood(wagon: &mut Wagon, gather_rates: &GatherRates) {
    wagon.wood_stock += gather_rates.wood;
}

pub fn perform_tasks(g_food: &u8, g_water: &u8, g_wood: &u8, repairers: &u8, gd: &mut GameData) {
    for _ in 0..*g_food {
        gather_food(&mut gd.wagon, &gd.gather_rates);
    }

    for _ in 0..*g_water {
        gather_water(&mut gd.wagon, &gd.gather_rates);
    }

    for _ in 0..*g_wood {
        gather_wood(&mut gd.wagon, &gd.gather_rates);
    }

    for _ in 0..*repairers {
        repair_wagon(&mut gd.wagon, &gd.gather_rates)
    }
}

/// This function performs the actions of a single worker repairing the wagon based on the repair_rate
pub fn repair_wagon(wagon: &mut Wagon, gather_rates: &GatherRates) {
    let max_durabilirt: u8 = 100;
    // let repair_tool = 10;
    if wagon.durability < max_durabilirt {
        wagon.durability += gather_rates.repair;
    }
}

// ? this is allowed?
pub fn assign_repairers(workers_left: &mut u8, repairers: &mut u8) {
    println!("How many workers shall repair the wagon?");
    loop {
        let player_input = get_input().parse::<u8>().unwrap();
        if player_input <= *workers_left {
            *repairers += player_input;
            *workers_left -= player_input;
            break;
        } else {
            println!("not enough workers to assign to task");
        }
    }
}

pub fn entertainment(people: &mut People, gather_rates: &GatherRates) {
    println!("Choose how you shall entertain: Feast, Stories, Music/Dance");
    let player_input = get_input();
    loop {
        if player_input == "Feast" {
            // add loop?
            people.morale += gather_rates.morale + 15;
            break;
        } else if player_input == "Music" {
            people.morale += gather_rates.morale + 10;
            break;
        } else if player_input == "Stories" {
            // add loop?
            people.morale += gather_rates.morale + 5;
            break;
        } else {
            println!("Huh?");
        }
    }
}

pub fn assign_gatherers(workers_left: &mut u8, g_food: &mut u8, g_water: &mut u8, g_wood: &mut u8) {
    println!("Assigning people to gathering tasks:");
    println!("How many workers shall be assigned to food?");
    loop {
        let player_input = get_input().parse::<u8>().unwrap();
        if player_input <= *workers_left {
            *g_food += player_input;
            *workers_left -= player_input;
            break;
        } else {
            println!("not enough workers to assign to task");
        }
    }

    println!("How many many workers shall be assigned to water?");
    loop {
        let player_input = get_input().parse::<u8>().unwrap();
        if player_input <= *workers_left {
            *g_water += player_input;
            *workers_left -= player_input;
            break;
        } else {
            println!("not enough workers to be assigned to water?");
        }
    }

    println!("How many many workers shall be assigned to wood?");
    loop {
        let player_input = get_input().parse::<u8>().unwrap();
        if player_input <= *workers_left {
            *g_wood += player_input;
            *workers_left -= player_input;
            break;
        } else {
            println!("not enough workers to be assigned to wood?");
        }
    }

    println!(
        "Food: {}, Water: {}, Wood: {}, Remaining: {}",
        g_food, g_water, g_wood, workers_left
    );
}

pub fn cmd_report(gd: &GameData) {
    let end_of_trail: u32;
    let total_miles: u32 = 2700;
    end_of_trail = total_miles - gd.miles_travelled;
    println!(
        "miles travelled: {}, miles to end of trial: {}",
        gd.miles_travelled, end_of_trail
    );
    println!(
        "food Stock: {}, wood stock: {} water stock: {}",
        gd.wagon.food_stock, gd.wagon.water_stock, gd.wagon.water_stock
    );
    println!("current population: {}", gd.people.population);
}

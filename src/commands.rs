use crate::structs::{trailpoint::TrailPoint, traits::Name, caravan::Caravan};

pub enum Command {
    Look,
    LookMap,
    NotFound,
    Quit,
    CheckCaravan,
    CheckAttire,
}

// ? Maybe rename this function with a more declarative name
fn do_command(cmd: Command, location: &TrailPoint, caravan: &Caravan) {
    match cmd {
        Command::Look => look(location),
        Command::LookMap => look_map(location),
        Command::NotFound => (),
        Command::Quit => std::process::exit(0),
        Command::CheckCaravan => check_caravan(caravan),
        Command::CheckAttire => check_attire(caravan),
        
    }
}

fn check_attire(caravan: &Caravan) {
    println!("{}", caravan.population_attire.display());
}

fn look(location: &TrailPoint) {
    println!("\nYou scan your surroundings:");
    println!("{}\n", location.get_description());
}

fn look_map(location: &TrailPoint) {
    println!(
        "According to the map, you are in {}",
        location.region.get_name()
    )
}

fn check_caravan(caravan: &Caravan) {
    println!("{}", caravan.display());
}



/// Converts user input to a Command enum
fn find_command(text: String) -> Command {
    if text == "look" {
        return Command::Look;
    } else if text == "map" {
        return Command::LookMap;
    } else if text == "quit" {
        return Command::Quit;
    } else if text == "checkCaravan" {
        return Command::CheckCaravan;
    }
    else if text == "checkAttire" {
        return Command::CheckAttire;
    }
    else {
        return Command::NotFound;
    }
}

pub fn process_command(input: String, location: &TrailPoint, caravan: &Caravan) {
    let cmd: Command = find_command(input);
    do_command(cmd, location, caravan);
}

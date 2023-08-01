use crate::structs::{trailpoint::TrailPoint, traits::Name};

pub fn look(location: &TrailPoint) {
    println!("\nYou scan your surroundings:");
    println!("{}\n", location.get_description());
}

pub fn look_map(location: &TrailPoint) {
    println!(
        "According to the map, you are in {}",
        location.region.get_name()
    )
}

pub enum Command {
    Look,
    LookMap,
    NotFound,
    Quit,
}

fn do_command(cmd: Command, location: &TrailPoint) {
    match cmd {
        Command::Look => look(location),
        Command::LookMap => look_map(location),
        Command::NotFound => (),
        Command::Quit => std::process::exit(0),
    }
}

fn find_command(text: String) -> Command {
    if text == "look" {
        return Command::Look;
    } else if text == "map" {
        return Command::LookMap;
    } else if text == "quit" {
        return Command::Quit;
    }
    else {
        return Command::NotFound;
    }
}

pub fn process_command(input: String, location: &TrailPoint) {
    let cmd: Command = find_command(input);
    do_command(cmd, location);
}

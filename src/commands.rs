use crate::structs::{trailpoint::TrailPoint, traits::Name, caravan::Caravan};

pub enum Command {
    Look,
    LookMap,
    NotFound,
    Quit,
    CheckCaravan,
    CheckAttire,
    CheckState,
    Camp,
}

// ? Maybe rename this function with a more declarative name
// ! so... many... mutable... references... can't be a good idea...
fn do_command(cmd: Command, location: &TrailPoint, caravan: &mut Caravan, game_state: &mut GameState) {
    match cmd {
        Command::Look => look(location),
        Command::LookMap => look_map(location),
        Command::NotFound => (),
        Command::Quit => std::process::exit(0),
        Command::CheckCaravan => check_caravan(caravan),
        Command::CheckAttire => check_attire(caravan),
        Command::CheckState => check_state(game_state),
        Command::Camp => camp(caravan, game_state),
    }
}
// ! fix this at a later date (never)
fn camp(caravan: &mut Caravan, game_state: &mut GameState) {
    // overnight events/deductions/replenishments take place
        // reduce food, water, wood by number of population
        // restore days_to_travel back to 12
        // incremet game day
        // inspect population
        // perform camp activities 
            // foraging
            // hunting
            // fishing
            // repairs
            // entertainment
    caravan.supplies.reduce_all_by(caravan.population);
    game_state.refresh_day();
    println!("You and the rest of the people make camp for the night...");



}

fn check_state(game_state: &GameState) {
    println!("{}", game_state.display_state());
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
    } else if text == "checkcaravan" {
        return Command::CheckCaravan;
    }
    else if text == "checkattire" {
        return Command::CheckAttire;
    }
    else if text == "checkstate" {
        return Command::CheckState;
    }
    else if text == "camp" {
        return Command::Camp; 
    }
    else {
        return Command::NotFound;
    }
}

pub fn process_command(input: String, location: &TrailPoint, caravan: &mut Caravan, game_state: &mut GameState) {
    let cmd: Command = find_command(input);
    do_command(cmd, location, caravan, game_state);
}


// ! move to its own module
pub struct GameState {
    pub days: u32,
    pub miles_travelled: u32,
    pub travel_hours_left_in_day: u8,
} 

impl GameState {
    pub fn display_state(&self) -> String {
        format!(
            "
            +--------------------------------------+
                Days Travelled: {days}
            +--------------------------------------+
                Miles Travelled: {miles}
            +--------------------------------------+
                Hours Before Nightfall {hours}
            +--------------------------------------+
            ",
            days=self.days, miles=self.miles_travelled, hours=self.travel_hours_left_in_day
        )
    }
    
    pub fn ok_to_travel(&self) -> bool {
        self.travel_hours_left_in_day > 1
    }
    
    pub fn reduce_day_hours(&mut self, amount: u8) {
        self.travel_hours_left_in_day -= amount;
    }

    pub fn increase_miles(&mut self, amount: u32) {
        self.miles_travelled += amount;
    }

    pub fn refresh_day(&mut self) {
        self.days += 1;
        self.travel_hours_left_in_day = 12;
    }
}
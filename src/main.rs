mod commands;
mod structs;

use commands::process_command;
use std::io;
use structs::{
    caravan::Caravan,
    trailpoint::{TrailPoint, _generate_trail}
};

fn main() {
    let trail: Vec<TrailPoint> = _generate_trail();
    let mut titer: std::slice::Iter<'_, TrailPoint> = trail.iter();
    let caravan = Caravan::new();

    loop {
        match titer.next() {
            Some(location) => loop {
                println!("Enter Command:");
                let input: String = get_input();

                if input == "travel".to_string() {
                    println!("You travel onward down the trail...");
                    break;
                } else {
                    process_command(input, &location, &caravan);
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
    String::from(input)
}

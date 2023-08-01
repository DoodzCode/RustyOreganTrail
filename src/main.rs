mod structs;
use std::io;
use uuid::Uuid;
use crate::structs::traits::{_generate_trail, look, where_am_i};

fn main() {
    let trail = _generate_trail();
    let mut titer = trail.iter();
    
    let mut count = 0;
    loop {
        match titer.next() {
            Some(location) => {
                count += 1;
                loop {
                    println!("\n{}\n", count);
                    println!("Enter Command:");
                    let inp = get_input();

                    if inp == "look".to_string() {
                        look(location);
                    }
                    else if inp == "where".to_string() {
                        where_am_i(location);
                    }
                    else if inp == "travel".to_string() {
                        break;
                    }
                    else if inp == "quit".to_string() {
                        std::process::exit(0);
                    }
                }
            }
            None => {break}
        }
    }


}

fn proccess_input() {
    
}

fn get_input() -> String {
    let mut r_input: String = String::new();
    io::stdin().read_line(&mut r_input).unwrap();
    let input: &str = r_input.trim();
    String::from(input)
}
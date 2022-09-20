pub mod days;
pub mod lib;

use regex::Regex;
use std::{collections::HashMap, env};

use crate::days::day_1::execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Supply a day argument. ie. `cargo run day1`".into());
    }

    let day = &args[1];

    let day_check = Regex::new(r"^day\d{1,2}$").unwrap();

    if day_check.is_match(day) == false {
        return Err("Supplied day was not correct, use 'day1' for example".into());
    }

    let mut methods = HashMap::new();

    methods.insert(String::from("day1"), execute);

    let method = methods.get(day);

    if let Some(m) = method {
        match m() {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

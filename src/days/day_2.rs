use std::collections::HashMap;

use crate::lib::load_file::load_data_file;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day2.txt")?;
    let lines: Vec<&str> = data.split("\n").collect();

    let mut directions = HashMap::new();

    directions.insert("horizontal", 0);
    directions.insert("depth", 0);

    for line in &lines {
        let split = line.split(' ');

        let direction = match split.clone().nth(0) {
            Some(d) => d,
            _ => "no_direction",
        };

        let steps: i32 = match split.clone().nth(1) {
            Some(d) => d.to_string().parse().unwrap(),
            _ => 0,
        };

        let (_, horizontal_value) = directions.get_key_value("horizontal").unwrap();
        let (_, depth_value) = directions.get_key_value("depth").unwrap();

        let mut horizontal: i32 = *horizontal_value;
        let mut depth: i32 = *depth_value;

        match direction {
            "forward" => horizontal = horizontal_value + steps,
            "down" => depth = depth_value + steps,
            "up" => depth = depth_value - steps,
            _ => (),
        }

        directions.insert("horizontal", horizontal);
        directions.insert("depth", depth);
    }

    let (_, horizontal_value) = directions.get_key_value("horizontal").unwrap();
    let (_, depth_value) = directions.get_key_value("depth").unwrap();

    println!("Q1 - {}", horizontal_value * depth_value);

    part_2(lines);

    Ok(())
}

fn part_2(lines: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut directions = HashMap::new();

    directions.insert("horizontal", 0);
    directions.insert("depth", 0);
    directions.insert("aim", 0);

    for line in lines {
        let split = line.split(' ');

        let direction = match split.clone().nth(0) {
            Some(d) => d,
            _ => "no_direction",
        };

        let steps: i32 = match split.clone().nth(1) {
            Some(d) => d.to_string().parse().unwrap(),
            _ => 0,
        };

        let (_, horizontal_value) = directions.get_key_value("horizontal").unwrap();
        let (_, depth_value) = directions.get_key_value("depth").unwrap();
        let (_, aim_value) = directions.get_key_value("aim").unwrap();

        let mut horizontal: i32 = *horizontal_value;
        let mut depth: i32 = *depth_value;
        let mut aim: i32 = *aim_value;

        match direction {
            "down" => aim += steps,
            "up" => aim -= steps,
            "forward" => {
                horizontal += steps;
                if aim > 0 {
                    depth += steps * aim;
                }
            }
            _ => (),
        }

        directions.insert("horizontal", horizontal);
        directions.insert("depth", depth);
        directions.insert("aim", aim);
    }

    let (_, horizontal_value) = directions.get_key_value("horizontal").unwrap();
    let (_, depth_value) = directions.get_key_value("depth").unwrap();

    println!("Q2 - {}", horizontal_value * depth_value);

    Ok(())
}

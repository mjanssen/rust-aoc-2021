use std::collections::HashMap;

use crate::lib::load_file::load_data_file;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day3.txt")?;
    let lines: Vec<&str> = data.split('\n').collect();

    // <index, (positive, negative)>
    let mut maps: HashMap<usize, (u32, u32)> = HashMap::new();

    let index_amount = lines[0].chars().as_str().len();
    let mut index = 0;

    loop {
        let data = calculate_occurence(&lines, index);
        maps.insert(index, data);

        index += 1;

        if index == index_amount {
            break;
        }
    }

    // sort maps hashmap
    let mut sorted: Vec<_> = maps.iter().collect();
    sorted.sort_by_key(|a| a.0);

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for (_, value) in sorted.iter() {
        let (positive, negative) = value;
        if positive > negative {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let gamma_number = i32::from_str_radix(gamma.as_str(), 2)?;
    let epsilon_number = i32::from_str_radix(epsilon.as_str(), 2)?;

    println!("Q1 - {}", gamma_number * epsilon_number);

    part_2(lines)?;

    Ok(())
}

fn calculate_occurence(lines: &Vec<&str>, char_index: usize) -> (u32, u32) {
    let mut positive: u32 = 0;
    let mut negative: u32 = 0;

    for line in lines {
        if let Some(char) = line.chars().nth(char_index) {
            let number: u8 = char.to_string().parse().unwrap();

            if number == 1 {
                positive += 1
            }
            if number == 0 {
                negative += 1
            }
        }
    }

    (positive, negative)
}

fn filter_items<'a>(items: Vec<&'a str>, check_index: usize, filter_number: u8) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for item in items {
        if let Some(char) = item.chars().nth(check_index) {
            let number: u8 = char.to_string().parse().unwrap();

            if number == filter_number {
                result.push(item);
            }
        }
    }

    result
}

fn part_2(lines: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut oxygen = lines.clone();
    let mut carbon = lines.clone();

    let index_amount = lines[0].chars().as_str().len();
    let mut index = 0;

    loop {
        if oxygen.len() > 1 {
            // calculate oxygen
            let (positive, negative) = calculate_occurence(&oxygen, index);

            let new_oxygen_vector: Option<Vec<&str>>;

            if positive >= negative {
                new_oxygen_vector = Some(filter_items(oxygen.clone(), index, 1));
            } else {
                new_oxygen_vector = Some(filter_items(oxygen.clone(), index, 0));
            }

            if let Some(nov) = new_oxygen_vector {
                oxygen = nov;
            }
        }

        // calculate carbon
        if carbon.len() > 1 {
            let (positive, negative) = calculate_occurence(&carbon, index);
            let new_carbon_vector: Option<Vec<&str>>;

            if negative <= positive {
                new_carbon_vector = Some(filter_items(carbon.clone(), index, 0));
            } else {
                new_carbon_vector = Some(filter_items(carbon.clone(), index, 1));
            }

            if let Some(ncv) = new_carbon_vector {
                carbon = ncv;
            }
        }
        // Increase the index for the for loop to check next char
        index += 1;

        if index >= index_amount {
            break;
        }
    }

    let mut oxygen_number: i32 = 1;
    if let Some(bytes) = oxygen.get(0) {
        oxygen_number = i32::from_str_radix(bytes, 2)?;
    }

    let mut carbon_number: i32 = 1;
    if let Some(bytes) = carbon.get(0) {
        carbon_number = i32::from_str_radix(bytes, 2)?;
    }

    println!("Q2 - {}", oxygen_number * carbon_number);

    Ok(())
}

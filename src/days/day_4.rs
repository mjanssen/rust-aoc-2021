use std::collections::HashMap;

use crate::lib::load_file::load_data_file;

struct Winner(u32, u32);

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day4_example.txt")?;
    let lines: Vec<&str> = data.split("\n").collect();

    // First line are throws seperated by comma
    let throw_line = &lines[0];

    // Rest of lines are the playing boards
    let boards = &lines[2..=lines.len() - 1];

    let mut horizontal_boards: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut vertical_boards: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();

    // Horizonta - Clean the first lines of the boards (trim spaces and store data correctly)
    let mut board_index = 0u32;
    for board_line in boards {
        if board_line.eq(&"") {
            board_index += 1;
            continue;
        }

        let mut existing_data: Vec<Vec<u32>> = match &horizontal_boards.get(&board_index) {
            Some(data) => data.to_vec(),
            _ => Vec::new(),
        };

        let board_line_cleaned = board_line.trim().replace("  ", " ");
        let mut line_vec: Vec<u32> = Vec::new();

        for number_string in board_line_cleaned.split(" ") {
            let should_append: u32 = number_string.parse().unwrap();
            line_vec.push(should_append);
        }

        existing_data.push(line_vec);

        // existing_data.push(board_line_cleaned);
        horizontal_boards.insert(board_index, existing_data);
    }

    // Vertical - Build vectors of all boards vertically
    for (board_index, board) in &horizontal_boards {
        let line_length = board.len();

        let mut vertical_board: Vec<Vec<u32>> = Vec::new();
        let mut number_index = 0u32;

        loop {
            let mut vertical_line: Vec<u32> = Vec::new();
            for lines in board {
                if let Some(num) = lines.get(number_index as usize) {
                    vertical_line.push(num.clone());
                }
            }

            vertical_board.push(vertical_line);

            number_index += 1;

            if number_index as usize == line_length {
                break;
            }
        }

        vertical_boards.insert(board_index.clone(), vertical_board);
    }

    let board_amount = horizontal_boards.len();
    let throws: Vec<&str> = throw_line.split(",").collect();

    let mut winner: Option<Winner> = None;

    for throw in &throws {
        let number: u32 = throw.parse().unwrap();

        let mut board_number = 0u32;

        if let Some(_) = winner {
            break;
        }

        loop {
            let mut line_counter = 0u32;

            // Check all board horitzontally
            let horizontal_board = match horizontal_boards.get(&board_number) {
                Some(board) => board,
                _ => panic!("Board not found"),
            };

            let mut clone = horizontal_board.clone();
            loop {
                let line: &mut Vec<u32> = &mut match clone.get(line_counter as usize) {
                    Some(d) => d.to_vec(),
                    _ => panic!("Line not found"),
                };

                if line.contains(&number) {
                    let index_element = line.iter().position(|&x| x == number).unwrap();
                    line.remove(index_element);

                    clone[line_counter as usize] = line.to_vec();
                }

                if line.len() == 0 {
                    winner = Some(Winner(number, board_number));
                    break;
                }

                line_counter += 1;
                if line_counter as usize >= clone.len() {
                    break;
                }
            }

            horizontal_boards.insert(board_number, clone);

            // Reset line counter for vertical checks
            line_counter = 0;

            // Check all board vertically
            let vertical_board = match vertical_boards.get(&board_number) {
                Some(board) => board,
                _ => panic!("Board not found"),
            };

            let mut clone = vertical_board.clone();
            loop {
                let line: &mut Vec<u32> = &mut match clone.get(line_counter as usize) {
                    Some(d) => d.to_vec(),
                    _ => panic!("Line not found"),
                };

                if line.contains(&number) {
                    let index_element = line.iter().position(|&x| x == number).unwrap();
                    line.remove(index_element);

                    clone[line_counter as usize] = line.to_vec();
                }

                if line.len() == 0 {
                    winner = Some(Winner(number, board_number));
                    break;
                }

                line_counter += 1;
                if line_counter as usize >= clone.len() {
                    break;
                }
            }

            vertical_boards.insert(board_number, clone);

            board_number += 1;
            if board_number as usize >= board_amount {
                break;
            }
        }
    }

    // Print data if there's a winner
    if let Some(w) = winner {
        let winning_throw = &w.0;
        let winning_board = &w.1;

        let board = match horizontal_boards.get(winning_board) {
            Some(b) => b,
            _ => panic!("{}", format!("Board not found {}", w.1)),
        };

        let mut sum = 0u32;

        for line in board {
            for num in line {
                sum += num
            }
        }

        println!("Q1 - {}", winning_throw * sum);
    }

    part_2(horizontal_boards, vertical_boards, throws);

    Ok(())
}

fn part_2(
    horizontal_boards: HashMap<u32, Vec<Vec<u32>>>,
    vertical_boards: HashMap<u32, Vec<Vec<u32>>>,
    throws: Vec<&str>,
) {
    let completed_boards: Vec<u32> = Vec::new();
    let mut board_index = 0u32;

    for throw in throws {
        let number: u32 = throw.parse().unwrap();

        println!("{}", number);
    }
}

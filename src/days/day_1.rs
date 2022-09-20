use crate::lib::load_file::load_data_file;

fn parse_number(line: &str) -> Option<u32> {
    match line.parse::<u32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day1.txt")?;
    let lines: Vec<&str> = data.split("\n").collect();

    let mut increases = 0;

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let current_line = parse_number(line);
        let previous_line = parse_number(lines[i - 1]);

        if let Some(curr_number) = current_line {
            if let Some(prev_number) = previous_line {
                if curr_number > prev_number {
                    increases += 1;
                }
            }
        }
    }

    println!("Q1 - {}", increases);

    part_2(&lines);

    Ok(())
}

fn part_2(lines: &Vec<&str>) {
    let mut increases = 0;
    let mut sums: Vec<u32> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if i + 3 < lines.len() {
            let numbers = [
                parse_number(line).expect("Expected 32 number"),
                parse_number(lines[i + 1]).expect("Expected 32 number"),
                parse_number(lines[i + 2]).expect("Expected 32 number"),
            ];
            let sum: u32 = numbers.iter().sum();
            sums.push(sum);
            if sums.len() > 1 {
                let previous_sum = sums[i - 1];
                if sum > previous_sum {
                    increases += 1;
                }
            }
        }
    }
    println!("Q2 - {}", increases);
}

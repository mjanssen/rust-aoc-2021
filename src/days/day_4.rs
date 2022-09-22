use crate::lib::load_file::load_data_file;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day4_example.txt")?;
    let lines: Vec<&str> = data.split("\n").collect();

    // First line are throws seperated by comma
    let throws = &lines[0];

    // Rest of lines are the playing boards
    let boards = &lines[2..=lines.len() - 1];

    println!("{:?}", throws);

    Ok(())
}

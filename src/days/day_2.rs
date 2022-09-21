use crate::lib::load_file::load_data_file;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day2.txt")?;

    Ok(())
}

use std::error::Error;
use crate::utils::get_first_arg;

pub fn read_from_path() -> Result<(), Box<dyn Error>> {
    // Open the CSV file.
    // - method 1
    // let file_path = get_first_arg()?;
    // let file = File::open(file_path)?;
    // let mut rdr = csv::Reader::from_reader(file);
    // - method 2
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    // csv::Reader::from_path will open the file 
    // and return an error if the file could not be opened

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
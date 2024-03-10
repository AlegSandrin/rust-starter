use std::{
    error::Error,
    io,
};

pub fn read_from_stdin() -> Result<(), Box<dyn Error>> {
    // Create a CSV parset that reads data from stdin.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    
    // Loop over each record.
    for result in rdr.records() {
        // If there was no problem in result, print the record.
        // Otherwise, print the error message and quit the program.
        
        // match result {
            //     Ok(record) => println!("{:?}", record),
        //     Err(err) => {
        //         println!("error reading CSV from <stdin>: {}", err);
        //         process::exit(1);
        //     }
        // }
        
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
use std::error::Error;
use crate::utils::utf8_encode as transcoded;

// Read CSV with UTF-8 enconding and print records with the header.
pub fn read_with_transcode() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(transcoded()?);

    let mut records_len: u16 = 0;
    let mut records_vec: Vec<csv::StringRecord> = Vec::new();

    for result in rdr.records() {
        records_len += 1;
        let record = result?;
        records_vec.push(record);
    }

    let length_chars_count: u16 = records_len.to_string().chars().count() as u16;
    let mut index: u16 = 0;

    for record in records_vec {
        
        let mut record_vec = Vec::new();
        
        for field in &record {
            record_vec.push(field);
        }

        let index_chars_count = index.to_string().chars().count() as u16;
        let diff = (length_chars_count - index_chars_count) as i16;
        let mut space = String::from(' ');
        if diff > 0 {
            for _ in 0..diff {
                space.push(' ');
            }
        }

        if index == 0 {
            // Print the header
            println!("{}==={:?}", space.replace(" ", "="), record_vec);
        } else {
            // Print the records
            println!("{}{}- {:?}", index, space, record_vec)
        }
        index += 1;
    }

    Ok(())
}
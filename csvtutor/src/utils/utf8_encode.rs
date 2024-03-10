use std::fs::File;
use crate::utils::get_first_arg;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{ DecodeReaderBytesBuilder, DecodeReaderBytes };

pub fn utf8_encode() -> Result<DecodeReaderBytes<File, Vec<u8>>, Box<dyn std::error::Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;

    // Read the CSV file with UTF-8 encoding
    // Reference:
    // https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    Ok(transcoded)
}

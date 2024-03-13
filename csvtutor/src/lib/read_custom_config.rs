use std::error::Error;
use crate::utils::utf8_encode as transcoded;

/*
    example.csv:
    "\"Hacksaw\" Jim Duggan";1987
    "Bret \"Hit Man\" Hart";1984
    # We're not sure when Rafael started, so omit the year.
    Rafael Halperin
    "\"Big Cat\" Ernie Ladd";1964
    "\"Macho Man\" Randy Savage";1985
    "Jake \"The Snake\" Roberts";1986

    To read this CSV data:
        1. Disable headers, since this data has none.
        2. Change the delimiter from , to ;.
        3. Change the quote strategy from doubled (e.g., "") to escaped (e.g., \").
        4. Permit flexible length records, since some omit the year.
        5. Ignore lines beginning with a #.
*/

pub fn read_custom_config() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        // .from_reader(io::stdin());
        .from_reader(transcoded()?);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
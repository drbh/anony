extern crate csv;
extern crate regex;
use regex::Regex;
use std::error::Error;
use std::io;
use std::process;

fn example() -> Result<(), Box<Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(rdr.headers()?)?;
    let re = Regex::new(r"[0-9]{3}-[0-9]{2}-[0-9]{4}").unwrap();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let mut x = vec![];
        // loop over each cell and check for match
        for cell in record.iter() {
            // make the cell into a managable obj
            let mut s = cell.to_string();
            if let Some(_i) = re.find(cell) {
                // now we reassign the value
                s = "XXX-XX-XXXX".to_string();
            }
            x.push(s)
        }
        // write back to std out
        wtr.write_record(&x)?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

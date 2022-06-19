extern crate csv;

use std::io;
use csv::Writer;

fn example() -> Result<(), Box<Error>> {
    let mut wtr = Writer::from_path("foo.csv")?;
    wtr.write_record(&["a", "b", "c"])?;
    wtr.write_record(&["x", "y", "z"])?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        let record = result.expect("a CSV record");

        println!("{:?}", record)
    }
   
}

extern crate csv;
extern crate time;
extern crate rand;
extern crate rustc_serialize;
extern crate byte_conv;
extern crate docopt;

mod reading;

use std::io::Write;
use docopt::Docopt;

const USAGE: &'static str = "
EMUSENSE is a utility program that generates fake sensor data and
saves it as both CSV and binary (.pps) file formats.

Usage:
    emusense <num-readings>
    emusense (-h | --help)
    emusense --version

Options:
    -h --help   Prints this message.
    --version   Prints version.
";

#[derive(Debug,RustcDecodable)]
struct Args {
    arg_num_readings: usize,
}

fn main() {
    // Parse command line arguments
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    // Generate fake readings
    // Reading( u64, i16, i16, i16 )
    let readings = reading::gen_readings(args.arg_num_readings);
    println!("Generated {} readings", args.arg_num_readings);
    // Save to CSV file
    println!("Saving as CSV...");
    let mut wtr = csv::Writer::from_file(std::path::Path::new("readings.csv")).unwrap();
    for r in readings.iter() {
        let res = wtr.encode(r);
        assert!(res.is_ok());
    }
    // Save to binary file
    println!("Saving as binary...");
    let mut f = std::fs::File::create("readings.pps").unwrap();
    let _ = f.write_all(&reading::readings_to_bytes(&readings));
}

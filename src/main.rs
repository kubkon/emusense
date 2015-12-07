extern crate csv;
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
    emusense <num-readings> [--rate=<hz>]
    emusense (-h | --help)
    emusense --version

Options:
    -h --help       Prints this message.
    --version       Prints version.
    --rate=<hz>     Sampling rate in Hz [default: 10].
";

#[derive(Debug,RustcDecodable)]
struct Args {
    arg_num_readings: usize,
    flag_rate: usize,
}

fn main() {
    // Parse command line arguments
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    // Generate fake readings
    // Reading( u16, i16, i16, i16 )
    let readings = reading::gen_readings(args.arg_num_readings, args.flag_rate);
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

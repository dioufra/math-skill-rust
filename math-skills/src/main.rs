use std::env;

use data::read_data_file;
use statistics::{average, median, standard_deviation, variance};
pub mod data;
pub mod statistics;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("USAGE: cargo run <filename>\nEx: cargo run data.txt");
        return;
    }
    let filename = &args[1];
    match read_data_file(filename) {
        Ok(data) => {
            println!("Average: {}", average(&data.to_vec()));
            println!("Median: {}", median(&data.to_vec()));
            println!("Variance: {}", variance(&data.to_vec()));
            println!("Standard Deviation: {}", standard_deviation(&data.to_vec()));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

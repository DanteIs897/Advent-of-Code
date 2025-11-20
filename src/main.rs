mod cli;
mod utils;
mod years;
mod errors;
mod models;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    let year = args.year.parse::<u16>().unwrap();
    let day = args.day.parse::<u8>().unwrap();
    
    if let Err(e) = years::dispatch(year, day) {
        eprintln!("Error: {}", e);
    }
}

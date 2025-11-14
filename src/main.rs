use clap::Parser;

mod utils;
mod y2024;

#[derive(Parser, Debug)]
#[command(
    name = "Advent of Code Runner",
    version = "1.0",
    about = "Ejecutor modular de retos de Advent of Code"
)]
struct Args {
    #[arg(short = 'y', long = "year")]
    year: String,
    
    #[arg(short = 'd', long = "day")]
    day: String,
}

fn main() {
    let args = Args::parse();
    let year_num: u16 = args.year.parse().expect("El año debe ser un número");
    let day_num: u8 = args.day.parse().expect("El día debe ser un número");
    
    match year_num {
        2024 => y2024::run_day(day_num),
        _ => eprintln!("Año {} no implementado.", year_num),
    }
}

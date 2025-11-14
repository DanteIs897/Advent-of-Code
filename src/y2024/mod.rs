pub mod day_01;

pub fn run_day(day: u8) {
  match day {
	1 => day_01::run(),
	_ => eprintln!("Día {} no implementado.", day),
  }
}

pub mod day_01;

use crate::errors::AocError;

pub fn run_day(day: u8) -> Result<(), AocError> {
  let result = match day {
	1 => day_01::run()?,
	_ => return Err(AocError::Unsupported),
  };
  
  println!("Respuesta 1: {}", result.part1);
  println!("Respuesta 2: {}", result.part2);
  
  Ok(())
}

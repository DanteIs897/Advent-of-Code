pub mod y2024;

use crate::errors::AocError;

pub fn dispatch(year: u16, day: u8) -> Result<(), AocError> {
  match year {
	2024 => y2024::run_day(day),
	_ => Err(AocError::Unsupported),
  }
}
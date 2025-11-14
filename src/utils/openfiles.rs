use std::path::PathBuf;
use std::fs;

pub fn open_input(year: u16, day: u8) -> String {
  let day_str = format!("{:02}", day);
  
  // Inputs path
  let mut path = PathBuf::from("inputs");
  path.push(format!("y{}", year));
  path.push(format!("{}.txt", day_str));
  
  fs::read_to_string(&path)
	.unwrap_or_else(|e| panic!(
	  "Failed to read input file for year {} day {}: {:?} (error: {})",
	  year, day, path, e))
}

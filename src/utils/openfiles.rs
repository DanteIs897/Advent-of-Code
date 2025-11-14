pub fn open_input(year: u16, day: u8) -> String {
  let day_str = format!("{:02}", day);
  let path = format!(
	"{}/src/y{}/inputs/{}.txt",
	env!("CARGO_MANIFEST_DIR"),
	year,
	day_str
  );
  
  std::fs::read_to_string(&path)
	.unwrap_or_else(|_| panic!("No se pudo leer el archivo de entrada: {}", path))
}

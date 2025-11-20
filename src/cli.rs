use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  #[arg(short = 'y', long = "year")]
  pub year: String,
  
  
  #[arg(short = 'd', long = "day")]
  pub day: String,
}
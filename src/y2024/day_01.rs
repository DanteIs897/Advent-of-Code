use crate::utils::openfiles::open_input;

pub fn run() {
  let input = open_input(2024, 1);
  println!("Día 01 - Entrada: {} líneas", input.lines().count());
  
  let mut left = Vec::new();
  let mut right = Vec::new();
  
  for line in input.lines() {
    let parts: Vec<&str> = line.split("   ").collect();
    
    if parts.len() == 2 {
      let l = parts[0].trim().parse::<i32>().unwrap();
      let r = parts[1].trim().parse::<i32>().unwrap();
      left.push(l);
      right.push(r);
    }
  }
  
  left.sort();
  right.sort();
  
  let mut total_diff = 0;
  for (l, r) in left.iter().zip(right.iter()) {
    let diff = if l > r {
      l - r
    } else {
      r - l
    };
    total_diff += diff;
  }
  
  println!("Respuesta 1: {}", total_diff);
  
  let mut sim_score = 0;
  
  for l in &left {
    let count = right.iter().filter(|&&r| r == *l).count();
    sim_score += *l * count as i32;
  }
  
  println!("Respuesta 2: {}", sim_score);
  
}

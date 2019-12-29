use std::fs;

pub fn calibration() {
  println!("calling from other file")
  let values = fs::read_to_string("../files/day1-numbers.txt")
  println("{}", values)
}
use std::fs;
use std::vec::Vec;

const YEAR : i32 = 2020;

pub fn main() {
  // 1. Process input
  let filename = std::env::args().nth(1).expect("No filename provided!");
  let contents = fs::read_to_string(filename).expect("Something went wrong with reading the file!");
  let numbers = contents.split("\n"); 
  println!("Step 1 done!");

  // 2. Vector of numbers
  let mut vec = Vec::new();
  for num in numbers {
    if num.chars().count() > 0 {
      vec.push(num.parse::<i32>().unwrap());
    }
  }
  vec.sort();
  println!("Step 2 done!");

  // 3. Find the desired numbers
  let mut done = 0;
  for elt in &vec {
    let ref1 : i32 = elt.clone();
    for elt2 in &vec {
      let ref2 : i32 = elt2.clone();
      for elt3 in &vec {
        if ref1 + ref2 + elt3 == YEAR {
          println!("Here are the numbers: {} and {} and {}", ref1, ref2, elt3);
          println!("Here is the answer: {}", ref1*ref2*elt3);
          done = 1;
          break;
        }
        if ref1 + ref2 + elt3 > 2020 {
          break;
        }
      }
      if done == 1 {
        break;
      }
    }
    if ref1 >= 1010 || done == 1{
    }
  }
  println!("Step 3 done!");
}

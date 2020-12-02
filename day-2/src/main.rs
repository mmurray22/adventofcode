use std::fs;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Policy {
  min: usize,
  max: usize,
  letter: String
}

pub fn create_policy (rule : String) -> Policy {
  let split_one : Vec<&str> = rule.split(" ").collect();
  let min_max : Vec<&str> = split_one[0].split("-").collect();
  Policy { min: min_max[0].parse::<usize>().unwrap(), 
           max: min_max[1].parse::<usize>().unwrap(), 
           letter: split_one[1].clone().to_string() }   
}

pub fn main() {
  // 1. Process input
  let filename = std::env::args().nth(1).expect("No filename provided!");
  let contents = fs::read_to_string(filename).expect("Something went wrong with reading the file!");
  let rules = contents.split("\n");
  println!("Step 1 done!");

  // 2. Separate policies and passwords
  let mut policy_vec = Vec::new();
  let mut passwd_vec = Vec::new();
  for rule in rules {
    if rule.chars().count() == 0 {
      break;
    }
    let pol_pass : Vec<&str> = rule.split(":").collect();
    policy_vec.push(create_policy(pol_pass[0].clone().to_string()));
    passwd_vec.push(pol_pass[1].clone().to_string());
  }
  println!("Step 2 done!");

  // 3. Check if passwords are ok
  let mut valid : i32 = 0;
  for (i, passwd) in passwd_vec.iter().enumerate() {
    let policy: Policy = policy_vec[i].clone();
    if policy.min <= passwd.matches(&policy.letter.clone()).count() && 
        policy.max >= passwd.matches(&policy.letter.clone()).count() {
      valid += 1;
    }
  }
  println!("Valid matches: {}", valid);
}

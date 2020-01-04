use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_fuel_for_mass(mass: i32) -> i32 {
  return cmp::max(0, (mass / 3) - 2);
}

fn get_total_fuel_for_mass(mass: i32) -> i32 {
  let mut total: i32 = 0; 
  let mut fuel_for_fuel: i32 = 0; 
  total += get_fuel_for_mass(mass);

  fuel_for_fuel = get_fuel_for_mass(total);
  while (fuel_for_fuel > 0){
    total += fuel_for_fuel;
    fuel_for_fuel = get_fuel_for_mass(fuel_for_fuel);
  }

  return total;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_fuel_for_mass() {
      assert_eq!(get_fuel_for_mass(3), 0);
      assert_eq!(get_fuel_for_mass(12), 2);
      assert_eq!(get_fuel_for_mass(14), 2);
      assert_eq!(get_fuel_for_mass(1969), 654);
      assert_eq!(get_fuel_for_mass(100756), 33583);
  }

  #[test]
  fn test_get_total_fuel_for_mass() {
      assert_eq!(get_total_fuel_for_mass(14), 2);
      assert_eq!(get_total_fuel_for_mass(1969), 966);
      assert_eq!(get_total_fuel_for_mass(100756), 50346);
  }
}

fn main() {
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line in reader.lines() {
      let mut fuel_for_fuel: i32 = 0;
      let mass = line.unwrap().to_string().parse::<i32>().unwrap();

      total += get_total_fuel_for_mass(mass); 
    }
    println!("total: {}", total);
}

use std::fs::File;
use std::io::{self, BufRead};

fn get_lines_from_file(file_name: &str) -> Result<Vec<String>, &'static str> {
  if let Ok(file) = File::open(file_name) {
    let mut result_list = vec!();
    for line_result in io::BufReader::new(file).lines() {
      if let Ok(line) = line_result {
        result_list.push(line);
      } else {
        return Err("Something went wrong while reading file.")
      }
    }
    Ok(result_list)
  } else {
    Err("")
  }
}

fn get_numbers_from_string_list(lines: Vec<String>) -> Result<Vec<i32>, &'static str> {
  let mut result_list: Vec<i32> = vec![];
  
  for line in lines {
    if let Ok(number) = line.parse::<i32>() {
      result_list.push(number);
    } else {
      return Err("File contains non number lines.");
    }
  }
  Ok(result_list)
}

fn count_number_of_going_down(numbers: Vec<i32>) -> i32 {
  let mut last = None;
  let mut increasing: i32 = 0;
  for number in numbers {
    if let Some(last_number) = last {
      if last_number < number {
        increasing = increasing + 1;
      }
    }
    last = Some(number);
  }
  return increasing;
}

fn main() {
  if let Ok(file_lines) = get_lines_from_file("./input") {
    if let Ok(number_list) = get_numbers_from_string_list(file_lines) {
      let increasing = count_number_of_going_down(number_list);
      println!("Number of increasing: {}", increasing);
    }
  }
}

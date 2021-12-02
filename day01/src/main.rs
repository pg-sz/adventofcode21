
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

fn count_number_of_going_up(numbers: &Vec<i32>) -> i32 {
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

fn get_sliding_window_list(numbers : Vec<i32>, window_size: usize) -> Result<Vec<i32>, &'static str> {

  if window_size > numbers.len() {
    return Err("window size is bigger than number of numbers");
  }

  let mut result_list = vec!();

  for index in 0..(numbers.len() - window_size + 1) {
    result_list.push(sum(&numbers[index .. index + window_size]));
  }

  return Ok(result_list);
}

fn sum(slice: &[i32]) -> i32 {
  return slice.iter().sum();
}

fn main() {
  if let Ok(file_lines) = get_lines_from_file("./input") {
    if let Ok(number_list) = get_numbers_from_string_list(file_lines) {
      let increasing = count_number_of_going_up(&number_list);
      println!("Number of increasing: {}", increasing);
      println!("{}", number_list.len());
      if let Ok(sliding_window_list) = get_sliding_window_list(number_list, 3) {
        let increasing_sliding_window = count_number_of_going_up(&sliding_window_list);
        println!("{}", sliding_window_list.len());
        println!("Number of increasing in sliding window: {}", increasing_sliding_window);
      }
    }
  }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::max;


struct DiagnosticReport {
  report: Vec<String>
}

impl DiagnosticReport {
  pub fn calculate_power_consumption(&self) -> Result<i32, &'static str> {
    if self.report.len() <= 0 {
      return Ok(0);
    }
    let row_width = {
      let mut max_length = 0;
      for line in &self.report {
        max_length = max(max_length, line.len());
      }
      max_length
    };

    if row_width <= 0 {
      return Err("line length is not greater than 0")
    }

    let mut counters: Vec<usize> = vec![0; row_width];

    for line in &self.report {
      for (index, character) in line.chars().enumerate() {
        if character == '1' {
          counters[index] += 1;
        }
      }
    }

    let number_of_rows = self.report.len() / 2;

    let mut gamma_string: String = String::from("");
    let mut epsilon_string: String = String::from("");

    println!("Number of rows: {}", number_of_rows);
    for counter in counters {
      print!("{}, ", counter);
      let (chararacter_gamma, character_epsilon) = if counter > number_of_rows {
        ('1', '0')
      } else {
        ('0', '1')
      };
      gamma_string.push(chararacter_gamma);
      epsilon_string.push(character_epsilon);
    }
    println!("{} {}", gamma_string, epsilon_string);
    let gamma = i32::from_str_radix(&*gamma_string, 2).unwrap();
    let epsilon = i32::from_str_radix(&*epsilon_string, 2).unwrap();

    return Ok(gamma * epsilon);
  }
}

trait DiagnosticReportLoader {
  fn load_report(&self) -> DiagnosticReport;
}


struct PlainFileStore {
  file_path: String
}

impl PlainFileStore {
  fn get_lines_from_file(&self) -> Result<Vec<String>, &'static str> {
    if let Ok(file) = File::open(&self.file_path) {
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
}

impl DiagnosticReportLoader for PlainFileStore {
    fn load_report(&self) -> DiagnosticReport {
      match self.get_lines_from_file() {
        Ok(lines) => DiagnosticReport { report: lines },
        Err(message) => {
          println!("Error: {}", message);
          DiagnosticReport { report: vec!() }
        }
      }
    }
}


fn main() {
  let plain_file_store = PlainFileStore { file_path: String::from("./input")};
  let report = plain_file_store.load_report();

  match report.calculate_power_consumption() {
    Ok(power_consumption) => println!("Power Consumption: {}", power_consumption),
    Err(message) => println!("{}", message)
  }
  
}

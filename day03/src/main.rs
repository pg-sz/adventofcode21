use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::max;


struct DiagnosticReport {
  report: Vec<String>
}

impl DiagnosticReport {

  fn get_row_width(&self) -> usize {
      let mut max_length = 0;
      for line in &self.report {
        max_length = max(max_length, line.len());
      }
      max_length
  }

  pub fn calculate_power_consumption(&self) -> Result<i32, &'static str> {
    if self.report.len() <= 0 {
      return Ok(0);
    }
    let row_width = self.get_row_width();

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
      let (chararacter_gamma, character_epsilon) = if counter > number_of_rows {
        ('1', '0')
      } else {
        ('0', '1')
      };
      gamma_string.push(chararacter_gamma);
      epsilon_string.push(character_epsilon);
    }
    let gamma = i32::from_str_radix(&*gamma_string, 2).unwrap();
    let epsilon = i32::from_str_radix(&*epsilon_string, 2).unwrap();

    return Ok(gamma * epsilon);
  }

  pub fn get_life_support_rating(&self) -> Result<i32, &'static str> {
    let oxygen = self.get_oxygen_reading()?;
    let co2 = self.get_co2_reading()?;

    return Ok(co2 * oxygen);
  }

  pub fn get_oxygen_reading(&self) -> Result<i32, &'static str> {
    match self.get_most_common_report_at(&self.report, 0) {
      Ok(most_common_report) => return Ok(i32::from_str_radix(&*most_common_report, 2).unwrap()),
      Err(message) => return Err(message)
    }
  }

  pub fn get_co2_reading(&self) -> Result<i32, &'static str> {
    match self.get_least_common_report_at(&self.report, 0) {
      Ok(least_common_report) => return Ok(i32::from_str_radix(&*least_common_report, 2).unwrap()),
      Err(message) => return Err(message)
    }
  }

  fn get_most_common_report_at(&self, report: &Vec<String>, offset: usize) -> Result<String, &'static str> {
    if self.get_row_width() < offset {
      return Err("Offset is to high.");
    }

    if report.len() > 1 {
      match self.split_report_at(report, offset) {
        Ok((ones, zeros)) =>
          if ones.len() >= zeros.len() {
            return self.get_most_common_report_at(&ones, offset + 1);
          } else {
            return self.get_most_common_report_at(&zeros, offset + 1);
          },
        Err(message) => Err(message)
      }
    } else if let Some(report_line) = report.get(0) {
      return Ok(String::from(report_line));
    } else {
      return Err("No report line found.");
    }
  }

  fn get_least_common_report_at(&self, report: &Vec<String>, offset: usize) -> Result<String, &'static str> {
    if self.get_row_width() < offset {
      return Err("Offset is to high.");
    }

    if report.len() > 1 {
      match self.split_report_at(report, offset) {
        Ok((ones, zeros)) => {
          if ones.len() < zeros.len() {
            return self.get_least_common_report_at(&ones, offset + 1);
          } else {
            return self.get_least_common_report_at(&zeros, offset + 1);
          } 
        },
        Err(message) => Err(message)
      }
    } else if let Some(report_line) = report.get(0) {
      return Ok(String::from(report_line));
    } else {
      return Err("No report line found.");
    }
  }
  

  fn split_report_at(&self, report: &Vec<String>, offset: usize) -> Result<(Vec<String>, Vec<String>), &'static str> {
    let mut ones = vec!();
    let mut zeros = vec!();

    for line in report {
      if let Some(character) = line.chars().nth(offset) {
        if character == '1' {
          ones.push(String::from(line));
        } else if character == '0' {
          zeros.push(String::from(line));
        } else{
          return Err("unkown character");
        }
      }
    }

    return Ok((ones, zeros));
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
  };

  match report.get_oxygen_reading() {
    Ok(oxygen_reading) => println!("{}", oxygen_reading),
    Err(message) => println!("{}", message)
  };

  match report.get_co2_reading() {
    Ok(co2_reading) => println!("{}", co2_reading),
    Err(message) => println!("{}", message)
  }

  match report.get_life_support_rating() {
    Ok(life_support_rating) => println!("{}", life_support_rating),
    Err(message) => println!("{}", message)
  }
  
}

#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn test_life_support_reading() {
    let plain_file_store = PlainFileStore { file_path: String::from("./test")};
    let report = plain_file_store.load_report();

    match report.get_life_support_rating() {
      Ok(life_support_rating) => assert_eq!(life_support_rating, 230),
      Err(_) => assert!(false, "Error occured")
    }
  }

    #[test]
  fn test_oxygen_reading() {
    let plain_file_store = PlainFileStore { file_path: String::from("./test")};
    let report = plain_file_store.load_report();

    match report.get_oxygen_reading(){
      Ok(oxygen_reading) => assert_eq!(oxygen_reading, 23, "O2 reading"),
      Err(_) => assert!(false, "Error occured")
    }
  }

  #[test]
  fn test_co2_reading() {
    let plain_file_store = PlainFileStore { file_path: String::from("./test")};
    let report = plain_file_store.load_report();
    
    match report.get_co2_reading() {
      Ok(co2_reading) => assert_eq!(co2_reading, 10, "CO2 reading"),
      Err(_) => assert!(false, "Error occured")
    }
  }
}

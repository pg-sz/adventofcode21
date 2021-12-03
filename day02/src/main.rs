use std::fs::File;
use std::io::{self, BufRead};
use std::fmt;


enum Direction {
  Forward(i32),
  Up(i32),
  Down(i32)
}

trait DirectionStore {
  fn load_directions(&self) -> Result<Vec<Direction>, &'static str>;
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

impl DirectionStore for PlainFileStore {
  fn load_directions(&self) -> Result<Vec<Direction>, &'static str> {
    match self.get_lines_from_file() {
      Ok(lines) => {
        let mut directions = vec!();
        for line in lines {
          if let Ok(direction) = Direction::from_string(line) {
            directions.push(direction);
          } else {
            return Err("line could not be parsed.");
          }
        } 
        return Ok(directions);
      },
      Err(x) => { return Err(x) }
    }
  }
}

impl Direction {
    fn from_string(line: String) -> Result<Direction, &'static str> {
      if let Some((direction, amount_string)) = line.split_once(" ") {
        if let Ok(amount) = amount_string.parse::<i32>() {
          match direction {
            "forward" => return Ok(Direction::Forward(amount)),
            "up" => return Ok(Direction::Up(amount)),
            "down" => return Ok(Direction::Down(amount)),
            &_ => return Err("unkown direction")
          }
        } else {
          return Err("Second column not a number");
        }
      } else {
        return Err("does not have two columns");
      }
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Direction::Forward(x) => return write!(f, "forward {}", x),
      Direction::Up(x) => return write!(f, "up {}", x),
      Direction::Down(x) => return write!(f, "down {}", x)
    }
  }
}

struct Submarine {
  horizontal: i32,
  vertical: i32,
  aim: i32
}

impl Submarine {
  fn zero() -> Submarine {
    return Submarine { horizontal:0, vertical: 0, aim: 0 };
  }

  fn apply_moves<T: DirectionStore>(&mut self, direction_store: &T) -> Result<(), &'static str> {
    match direction_store.load_directions() {
      Ok(directions) => {
        for direction in directions {
          self.move_to(&direction);
        }
        return Ok(());
      },
      Err(x) => { return Err(x) }
    }
  }

  fn move_to(&mut self, direction: &Direction) {
    match direction {
      Direction::Forward(x) => {
        self.horizontal += x;
        self.vertical += self.aim * x
      },
      Direction::Down(x) => self.aim += x,
      Direction::Up(x) => self.aim -= x
    }
  }

  fn calculate_sweeped_area(&self) -> i32 {
    self.horizontal * self.vertical
  }
}

impl fmt::Display for Submarine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Submarine {{ horizontal:{}, vertical:{}, aim:{} }}", self.horizontal, self.vertical, self.aim)
  }
}



fn main() {
    let mut submarine = Submarine::zero();
    let plain_file_store = PlainFileStore { file_path: String::from("./input") };
    
    if let Err(error_message) = submarine.apply_moves(&plain_file_store) {
      println!("{}", error_message);
      return;
    }
    println!("{}", submarine);
    println!("Sweeped area: {}", submarine.calculate_sweeped_area());
}

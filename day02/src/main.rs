use std::fs::File;
use std::io::{self, BufRead};
use std::fmt;

enum Direction {
  Forward(i32),
  Up(i32),
  Down(i32)
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
}

impl fmt::Display for Submarine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Submarine(h={}, v={}, a={})", self.horizontal, self.vertical, self.aim)
  }
}

fn parse_line(line: String) -> Result<Direction, &'static str> {
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

fn parse_lines(lines: Vec<String>) -> Result<Vec<Direction>, &'static str> {
  let mut directions = vec!();
  for line in lines {
    if let Ok(direction) = parse_line(line) {
      directions.push(direction);
    } else {
      return Err("line could not be parsed.");
    }
  }
  Ok(directions)
}

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

fn main() {
    let mut submarine = Submarine::zero();

    if let Ok(lines) = get_lines_from_file("./input") {
      if let Ok(directions) = parse_lines(lines) {
        for direction in directions{
          submarine.move_to(&direction);
        }
      }
    }

    println!("{}", submarine);
    println!("{}", submarine.horizontal * submarine.vertical);
}

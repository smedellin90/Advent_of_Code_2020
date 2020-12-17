use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::vec::Vec;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

struct Password {
    password: String,
    min: usize,
    max: usize,
    search_char: char,
}

impl Password {
    pub fn is_valid_password(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| self.is_valid_password_char(c))
            .count();
        count >= self.min && count <= self.max
    }

    fn is_valid_password_char(&self, char: &char) -> bool {
        self.search_char == *char
    }
}

fn parse_line(line: &str) -> Password {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(\w+)$").expect("Incorrect Regular Expression");
    let captures = re.captures(line).expect("No captures");

    Password {
        min: str::parse(captures.get(1).unwrap().as_str()).unwrap(),
        max: str::parse(captures.get(2).unwrap().as_str()).unwrap(),
        search_char: str::parse(captures.get(3).unwrap().as_str()).unwrap(),
        password: str::parse(captures.get(4).unwrap().as_str()).unwrap(),
    }
}

fn parse(input: &str) -> Vec<Password> {
    input.lines().map(|v| parse_line(v)).collect()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut vec: Vec<Password> = Vec::new();
    vec = parse(&input);
    println!("{}", vec.iter().filter(|x| x.is_valid_password()).count());
    Ok(())
}

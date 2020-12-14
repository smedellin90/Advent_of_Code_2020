use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::vec::Vec;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut vec = Vec::new();
    let mut indeces: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let change: i32 = line.parse()?;
        vec.push(change);
    }

    const OBJECTIVE: i32 = 2020;
    let mut i_idx = 0;

    loop {
        if i_idx == vec.len() || !indeces.is_empty() {
            break;
        }

        for number in i_idx..vec.len() {
            let sum = vec[i_idx] + vec[number];
            if sum == OBJECTIVE {
                indeces.insert((vec[i_idx], vec[number]));
            }
        }

        for number in 0..i_idx {
            let sum = vec[i_idx] + vec[number];
            if sum == OBJECTIVE {
                indeces.insert((vec[i_idx], vec[number]));
            }
        }

        i_idx += 1;
    }

    for item in &indeces {
        println!("{:?}", item.0 * item.1);
    }

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut vec = Vec::new();
    let mut indeces: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in input.lines() {
        let change: i32 = line.parse()?;
        vec.push(change);
    }

    const OBJECTIVE: i32 = 2020;

    let i_idx = 0;
    let j_idx = i_idx + 1;
    let k_idx = j_idx + 1;
    let mut found:bool = false;

    for i in i_idx..vec.len() - 2 {
        if found { break; }
        for j in j_idx..vec.len() - 1 {
            if found { break; }
            for k in k_idx..vec.len() {
                if vec[i] + vec[j] + vec[k] == OBJECTIVE {
                    indeces.insert((vec[i], vec[j], vec[k]));
                    found = true;
                    break;
                }
            }
        }
    }
    for item in &indeces {
        println!("{:?}", item.0 * item.1 * item.2);
    }
    Ok(())
}

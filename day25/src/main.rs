use clap::{crate_description, App, Arg};
use day25::*;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("File with puzzle input")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!(crate_description!());

    let (key1, key2) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    println!("Part 1: {}", part1(key1, key2));
}

fn read_input(filename: &str) -> Result<(u64, u64), String> {
    let input = read_to_string(filename).map_err(|err| err.to_string())?;
    let numbers = &mut input
        .lines()
        .zip(1..)
        .map(|(line, line_num)| {
            line.parse().map_err(|err: ParseIntError| {
                format!("Line {}: {}", line_num, err.to_string())
            })
        })
        .take(2)
        .collect::<Result<Vec<_>, _>>()?;
    match numbers.get(0..2) {
        Some(&[key1, key2]) => Ok((key1, key2)),
        _ => Err("Missing public key(s)".to_string()),
    }
}

use clap::{crate_description, App, Arg};
use day09::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    let input = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    let preamble_len = 25;
    match part1(&input, preamble_len) {
        Some(result1) => {
            println!("Part 1: {}", result1);
            match part2(&input, result1) {
                Some(result2) => println!("Part 2: {}", result2),
                None => println!("Part 2: not found"),
            };
        }
        None => {
            println!("Part 1: not found\nPart 2: not found");
        }
    };
}

fn read_input(filename: &str) -> Result<Vec<i64>, String> {
    let input_file = File::open(filename).map_err(|err| err.to_string())?;

    BufReader::new(input_file)
        .lines()
        .zip(1..)
        .map(|(line, line_num)| {
            line.map_err(|err| (line_num, err.to_string()))
                .and_then(|value| {
                    value.parse().map_err(|err: ParseIntError| {
                        (line_num, err.to_string())
                    })
                })
        })
        .collect::<Result<_, _>>()
        .map_err(|(line_num, err)| format!("Line {}: {}", line_num, err))
}

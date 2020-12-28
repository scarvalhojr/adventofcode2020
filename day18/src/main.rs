use clap::{crate_description, App, Arg};
use day18::*;
use std::fs::read_to_string;
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

    match part1(&input) {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => println!("Part 1: {}", err),
    };
    match part2(&input) {
        Ok(result) => println!("Part 2: {}", result),
        Err(err) => println!("Part 2: {}", err),
    };
}

fn read_input(filename: &str) -> Result<Vec<String>, String> {
    read_to_string(filename)
        .map(|input| input.lines().map(|line| line.to_string()).collect())
        .map_err(|err| err.to_string())
}

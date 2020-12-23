use clap::{crate_description, App, Arg};
use day20::*;
use std::fs::read_to_string;
use std::process::exit;
use tile::*;

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

    let tiles = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    match part1(&tiles) {
        Some(result) => println!("Part 1: {}", result),
        None => println!("Part 1: not found"),
    };
    match part2(&tiles) {
        Ok(result) => println!("Part 2: {}", result),
        Err(err) => println!("Part 2: {}", err),
    };
}

fn read_input(filename: &str) -> Result<Vec<Tile>, String> {
    let input = read_to_string(filename).map_err(|err| err.to_string())?;
    input
        .split("\n\n")
        .filter(|block| !block.trim().is_empty())
        .map(|block| block.parse())
        .collect::<Result<Vec<_>, _>>()
}

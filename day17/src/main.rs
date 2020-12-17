use clap::{crate_description, App, Arg};
use part1::*;
use part2::*;
use std::fs::read_to_string;
use std::process::exit;

pub mod part1;
pub mod part2;

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

    let (grid3d, grid4d) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    println!("Part 1: {}", part1(&grid3d));
    println!("Part 2: {}", part2(&grid4d));
}

fn read_input(filename: &str) -> Result<(Grid3D, Grid4D), String> {
    let input = read_to_string(filename).map_err(|err| err.to_string())?;
    let grid3d = input.parse()?;
    let grid4d = input.parse()?;
    Ok((grid3d, grid4d))
}

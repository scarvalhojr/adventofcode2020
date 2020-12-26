use clap::{crate_description, App, Arg};
use day22::*;
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

    let (cards1, cards2) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok((cards1, cards2)) => (cards1, cards2),
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    println!("Part 1: {}", part1(&cards1, &cards2));
    println!("Part 2: {}", part2(&cards1, &cards2));
}

fn read_input(filename: &str) -> Result<(Cards, Cards), String> {
    let input = read_to_string(filename).map_err(|err| err.to_string())?;
    let lines = &mut input.lines().zip(1..);
    let cards1 = lines
        .skip(1)
        .take_while(|(line, _)| !line.trim().is_empty())
        .map(|(line, line_num)| {
            line.parse().map_err(|err: ParseIntError| {
                format!("Line {}: {}", line_num, err.to_string())
            })
        })
        .collect::<Result<_, _>>()?;
    let cards2 = lines
        .skip(1)
        .take_while(|(line, _)| !line.trim().is_empty())
        .map(|(line, line_num)| {
            line.parse().map_err(|err: ParseIntError| {
                format!("Line {}: {}", line_num, err.to_string())
            })
        })
        .collect::<Result<_, _>>()?;
    Ok((cards1, cards2))
}

use clap::{crate_description, App, Arg};
use day13::*;
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

    let (time, busses) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok((time, busses)) => (time, busses),
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    match part1(time, &busses) {
        Some(result) => println!("Part 1: {}", result),
        None => println!("Part 1: not found"),
    };
    println!("Part 2: {}", part2(&busses));
}

fn read_input(filename: &str) -> Result<(Timestamp, Vec<Option<Bus>>), String> {
    let input = read_to_string(filename).map_err(|err| err.to_string())?;
    let mut lines = input.lines();

    let timestamp = lines
        .next()
        .ok_or_else(|| String::from("Missing timestamp"))?
        .parse()
        .map_err(|err| format!("Invalid timestamp: {}", err))?;
    let busses = lines
        .next()
        .ok_or_else(|| String::from("Missing bus list"))?
        .split(',')
        .map(|bus| {
            if bus == "x" {
                Ok(None)
            } else {
                bus.parse()
                    .map(Some)
                    .map_err(|err| format!("Invalid bus number: {}", err))
            }
        })
        .collect::<Result<_, _>>()?;

    Ok((timestamp, busses))
}

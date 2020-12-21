use clap::{crate_description, App, Arg};
use day19::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    let (rules, messages) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    match part1(&rules, &messages) {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => println!("Part 1: {}", err),
    };
    match part2(&rules, &messages) {
        Ok(result) => println!("Part 2: {}", result),
        Err(err) => println!("Part 2: {}", err),
    };
}

fn read_input(filename: &str) -> Result<(Vec<Rule>, Vec<String>), String> {
    let input_file = File::open(filename).map_err(|err| err.to_string())?;
    let lines = &mut BufReader::new(input_file).lines().zip(1..);
    let rules = lines
        .take_while(|(line, _)| {
            line.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(true)
        })
        .map(|(line, line_num)| {
            line.map_err(|err| (line_num, err.to_string()))
                .and_then(|value| value.parse().map_err(|err| (line_num, err)))
        })
        .collect::<Result<_, _>>()
        .map_err(|(line_num, err)| format!("Line {}: {}", line_num, err))?;
    let messages = lines
        .map(|(line, line_num)| line.map_err(|err| (line_num, err.to_string())))
        .collect::<Result<_, _>>()
        .map_err(|(line_num, err)| format!("Line {}: {}", line_num, err))?;

    Ok((rules, messages))
}

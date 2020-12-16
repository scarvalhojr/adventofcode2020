use clap::{crate_description, App, Arg};
use day16::*;
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

    let (rules, your_ticket, nearby_tickets) =
        match read_input(args.value_of("INPUT").unwrap()) {
            Ok(data) => data,
            Err(err) => {
                println!("Failed to read input: {}", err);
                exit(2);
            }
        };

    println!("Part 1: {}", part1(&rules, &nearby_tickets));
    match part2(&rules, &your_ticket, &nearby_tickets) {
        Some(result) => println!("Part 2: {}", result),
        None => println!("Part 2: not found"),
    };
}

fn read_input(
    filename: &str,
) -> Result<(Vec<Rule>, Ticket, Vec<Ticket>), String> {
    let input = read_to_string(filename).map_err(|err| err.to_string())?;
    let lines = &mut input.lines().zip(1..);
    let rules = lines
        .take_while(|(line, _)| !line.trim().is_empty())
        .map(|(line, line_num)| {
            line.parse()
                .map_err(|err| format!("Line {}: {}", line_num, err))
        })
        .collect::<Result<_, _>>()?;
    let your_ticket = lines
        .skip_while(|(line, _)| line.trim().is_empty())
        .nth(1)
        .ok_or_else(|| String::from("Missing 'your ticket'"))
        .and_then(|(line, line_num)| {
            parse_ticket(line)
                .map_err(|err| format!("Line {}: {}", line_num, err))
        })?;
    let nearby_tickets = lines
        .skip_while(|(line, _)| line.trim().is_empty())
        .skip(1)
        .take_while(|(line, _)| !line.trim().is_empty())
        .map(|(line, line_num)| {
            parse_ticket(line)
                .map_err(|err| format!("Line {}: {}", line_num, err))
        })
        .collect::<Result<_, _>>()?;

    Ok((rules, your_ticket, nearby_tickets))
}

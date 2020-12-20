use clap::{crate_description, App, Arg};
use day18::*;
use part1::*;
use part2::*;
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

    let (expr_v1, expr_v2) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    println!("Part 1: {}", sum_all(&expr_v1));
    println!("Part 2: {}", sum_all(&expr_v2));
}

fn read_input(
    filename: &str,
) -> Result<(Vec<Expression>, Vec<Expression>), String> {
    let input = read_to_string(filename).map_err(|err| err.to_string())?;
    let expressions_v1 = input
        .lines()
        .map(|line| parse_expression_v1(line))
        .collect::<Result<_, _>>()?;
    let expressions_v2 = input
        .lines()
        .map(|line| parse_expression_v2(line))
        .collect::<Result<_, _>>()?;
    Ok((expressions_v1, expressions_v2))
}

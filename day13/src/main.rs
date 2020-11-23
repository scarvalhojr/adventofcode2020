use clap::{crate_description, App, Arg};
use day13::{part1, part2};
use std::fs::File;
use std::io::BufReader;
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
    let _input = read_input(args.value_of("INPUT").unwrap());
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn read_input(filename: &str) -> i32 {
    let _file = match File::open(filename) {
        Ok(file) => BufReader::new(file),
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    unimplemented!()
}

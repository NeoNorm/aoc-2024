
use day05::*;
use clap::{Arg, Command, value_parser};

fn main() {
    let args = Command::new("AOC").arg(Arg::new("part").required(true).value_parser(value_parser!(usize)));
    let matches = args.get_matches();
    let part = matches.get_one::<usize>("part");

    let result = match part {
        Some(1) => format!("{}", part1::go_1(INPUT)),
        Some(2) => format!("{}", part2::go_1(INPUT)),
        _ => "Invalid part".to_string(),
    };

    println!("{}", result);
}

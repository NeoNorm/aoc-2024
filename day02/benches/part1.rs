use day02::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn test_1() {
    part1::go_1(divan::black_box(INPUT));
}


use day01::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_1() {
    part1::go_1(divan::black_box(INPUT));
}

#[divan::bench]
fn part1_2() {
    part1::go_2(divan::black_box(INPUT));
}


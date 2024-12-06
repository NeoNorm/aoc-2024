use day04::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_1() {
    part1::go_1(divan::black_box(INPUT));
}

#[divan::bench]
fn bench_2() {
    part1::go_2(divan::black_box(INPUT));
}

#[divan::bench]
fn bench_3() {
    part1::go_3(divan::black_box(INPUT));
}

use day01::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn test_1() {
    part2::go_1(divan::black_box(INPUT));
}

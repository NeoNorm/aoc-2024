

use day07::part2::*;
use day07::INPUT;

fn main() {
    divan::main();
}

#[divan::bench]
fn first() {
    go(divan::black_box(INPUT));
}

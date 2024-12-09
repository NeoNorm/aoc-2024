

use day08::part2::*;
use day08::INPUT;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench() {
    go(divan::black_box(INPUT));
}

use day_01::*;

fn main() {
    // Runs all benchmarks
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input1.txt"))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input2.txt"))).unwrap();
}

#[divan::bench]
fn part2_biscardi() {
    part2_biscardi::process(divan::black_box(include_str!("../input2.txt"))).unwrap();
}
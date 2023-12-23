use day_10::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_bits() {
    part1::process_bits(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2_bits() {
    part2::process_bits(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}
use day_18::*;

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
fn part1_bytes() {
    part1::process_bytes(divan::black_box(include_str!(
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
fn part2_bytes() {
    part2::process_bytes(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}
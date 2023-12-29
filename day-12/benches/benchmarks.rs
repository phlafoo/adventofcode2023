use day_12::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1_naive() {
    part1::process_naive(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}

#[divan::bench(sample_size = 10, sample_count = 100)]
fn part1() {
    part1::process(divan::black_box(include_str!(
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
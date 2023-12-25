use day_11::*;

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
fn part1_faster() {
    part1::process_faster(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_fastest() {
    part1::process_fastest(divan::black_box(include_str!(
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
fn part2_fastest() {
    part2::process_fastest(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}
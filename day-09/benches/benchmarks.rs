use day_09::*;

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
fn part1_optimized() {
    part1::process_optimized(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_optimized_dft() {
    part1::process_optimized_dft(divan::black_box(include_str!(
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
fn part2_dft() {
    part2::process_dft(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}
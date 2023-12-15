use day_05::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_optimized() {
    part1::process_optimized(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2_optimized() {
    part2::process_optimized(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}

// VERY SLOW ~1.5 min
// #[divan::bench(sample_size = 1, sample_count = 1)]
// fn part2() {
//     part2::process(divan::black_box(include_str!(
//         "../input2.txt",
//     )))
//     .unwrap();
// }
use day_11::part1::*;
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input.txt");
    // let result = process(file).context("process part 1")?;
    let result = process_fastest(file).context("process part 1")?;
    // 9214785
    println!("{}", result);
    Ok(())
}
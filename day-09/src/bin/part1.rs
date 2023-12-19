use day_09::part1::*;
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 1")?;
    // let result = process_optimized(file).context("process part 1")?;
    // let result = process_optimized_dft(file).context("process part 1")?;
    // 1987402313
    println!("{}", result);
    Ok(())
}
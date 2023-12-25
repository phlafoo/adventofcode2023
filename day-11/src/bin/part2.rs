use day_11::part2::*;
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input.txt");
    // let result = process(file).context("process part 2")?;
    let result = process_fastest(file).context("process part 2")?;
    // 613686987427
    println!("{}", result);
    Ok(())
}

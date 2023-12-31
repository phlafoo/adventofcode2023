use day_12::part2::*;
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    // 6720660274964
    println!("{}", result);
    Ok(())
}

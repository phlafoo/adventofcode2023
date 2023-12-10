use day_01::part2_biscardi::process;
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 2 biscardi")?;
    println!("{}", result);
    Ok(())
}
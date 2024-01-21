use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input.txt");
    // let result = day_19::part2::process(file).context("process part 2")?;
    let result = day_19::part2_dfs::process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}

use advent_of_code_data::{get_input, submit_answer, Day, Part, Year};

fn main() -> anyhow::Result<()> {
    // Fetches the puzzle input for Day 1 in 2025.
    let input = get_input(Day(1), Year(2025))?;
    println!("{}", input);

    // Submits an answer for Day 1 in 2025, part 1.
    submit_answer(42.into(), Part::One, Day(1), Year(2025))?;

    Ok(())
}

mod day1;
mod day2;
mod day3;
mod utils;

use anyhow::Context;
use std::error::Error;
use structopt::StructOpt;
use utils::Solution;

#[derive(StructOpt)]
struct Cli {
    day: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let day = Cli::from_args().day;
    let filename = format!("input/day{}.txt", day);
    let content: Vec<String> = utils::read_lines(filename)
        .with_context(|| format!("could not read input file for day {}", day))?;

    let solution = match day {
        1 => Solution::new::<day1::SonarSweep>(content),
        2 => Solution::new::<day2::Dive>(content),
        3 => Solution::new::<day3::BinaryDiagnostic>(content),
        _ => unreachable!(),
    };

    solution.get_result(day);

    Ok(())
}

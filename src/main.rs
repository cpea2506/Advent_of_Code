mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

use anyhow::Context;
use std::error::Error;
use structopt::StructOpt;
use utils::Solution;

#[derive(StructOpt)]
struct Cli {
    day: u32,

    #[structopt(short, long, help = "Uses example file provided by AOC")]
    example: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();

    let main_file = if cli.example { "example" } else { "input" };
    let filename = format!("src/day{}/{}.txt", cli.day, main_file);

    let content: Vec<String> = utils::read_lines(filename)
        .with_context(|| format!("could not read {} file for day {}", main_file, cli.day))?;

    let solution = match cli.day {
        1 => Solution::new::<day1::SonarSweep>(content),
        2 => Solution::new::<day2::Dive>(content),
        3 => Solution::new::<day3::BinaryDiagnostic>(content),
        4 => Solution::new::<day4::GiantSquid>(content),
        _ => unreachable!(),
    };

    solution.get_result(cli.day);

    Ok(())
}

use avent::Solutions;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "AOC day", default_value = "1")]
    day: u8,

    #[structopt(short, long, help = "Uses example file provided by AOC")]
    example: bool,

    #[structopt(short, long, help = "Gets all solutions for all AOC days")]
    all: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let main_file = if args.example { "example" } else { "input" };

    if args.all {
        (1..=9).for_each(|d| Solutions::get_result(main_file, d));
    } else {
        Solutions::get_result(main_file, args.day);
    }

    Ok(())
}

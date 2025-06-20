use advent2023::get_solution;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(help = "AOC day", default_value = "1")]
    day: u8,

    #[arg(short, long, help = "Uses example file provided by AOC")]
    example: bool,

    #[arg(short, long, help = "Gets all solutions for all AOC days")]
    all: bool,
}

fn main() {
    let day_count = 1;
    let args = Cli::parse();
    let main_file = if args.example { "example" } else { "input" };

    if args.all {
        (1..=day_count).for_each(|d| get_solution(main_file, d));
    } else {
        get_solution(main_file, args.day);
    }
}

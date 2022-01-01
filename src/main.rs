mod day1;
mod day2;
mod day3;

use anyhow::Context;
use humantime::format_duration;
use owo_colors::OwoColorize;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    day: u32,
}

pub trait Avent {
    fn new(data: Vec<String>) -> Self
    where
        Self: Sized;
    fn part1(&self) -> usize;
    fn part2(&self) -> usize;
}

struct Solution {
    event: Box<dyn Avent>,
    time: Duration,
}

impl Solution {
    fn new<Event: Avent + 'static>(content: Vec<String>) -> Self {
        let (event, time) = get_time(|| Event::new(content));

        Solution {
            event: Box::new(event),
            time,
        }
    }

    fn get_result(&self, day: u32) {
        let (part1, time1) = get_time(|| self.event.part1());
        let (part2, time2) = get_time(|| self.event.part2());

        println!("Solution for day {}", day);
        println!(
            "Collect data in {}",
            format_duration(self.time).fg_rgb::<255, 63, 128>()
        );
        println!(
            "part 1: {} in {}",
            part1.fg_rgb::<100, 252, 218>(),
            format_duration(time1).fg_rgb::<100, 252, 218>()
        );
        println!(
            "part 2: {} in {}",
            part2.fg_rgb::<100, 252, 218>(),
            format_duration(time2).fg_rgb::<100, 252, 218>()
        );
    }
}

fn get_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let time = start.elapsed();

    (result, time)
}

fn main() -> Result<(), Box<dyn Error>> {
    let day = Cli::from_args().day;
    let file = File::open(format!("input/day{}.txt", &day))
        .with_context(|| format!("could not read input file for day {}", day))?;
    let content: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let solution = match day {
        1 => Solution::new::<day1::SonarSweep>(content),
        2 => Solution::new::<day2::Dive>(content),
        3 => Solution::new::<day3::BinaryDiagnostic>(content),
        _ => unreachable!(),
    };

    solution.get_result(day);

    Ok(())
}

mod day1;
mod day2;

use anyhow::Context;
use humantime::format_duration;
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
}

impl Solution {
    fn new<Event: Avent + 'static>(content: Vec<String>) -> Self {
        let event = Event::new(content);

        Solution {
            event: Box::new(event),
        }
    }

    fn get_result(&self, day: u32) {
        let (part1, time1) = get_time(|| self.event.part1());
        let (part2, time2) = get_time(|| self.event.part2());

        println!("Solution for day {}", day);
        println!("part 1: {}, time: {}", part1, format_duration(time1));
        println!("part 2: {}, time: {}", part2, format_duration(time2));
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
        _ => unreachable!(),
    };

    solution.get_result(day);

    Ok(())
}

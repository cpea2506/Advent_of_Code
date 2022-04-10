mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use anyhow::Context;
use humantime::format_duration;
use owo_colors::OwoColorize;
use std::{
    fs,
    time::{Duration, Instant},
};

trait Avent {
    fn new(data: &str) -> Self
    where
        Self: Sized;
    fn day() -> u8
    where
        Self: Sized;
    fn part1(&self) -> usize;
    fn part2(&self) -> usize;
}

struct Solution {
    event: Box<dyn Avent>,
    time: Duration,
    day: u8,
}

impl Solution {
    fn new<Event: Avent + 'static>(content: &str) -> Self {
        let (event, time) = get_time(|| Event::new(content));

        Solution {
            event: Box::new(event),
            time,
            day: Event::day(),
        }
    }

    fn get_result(&self) {
        let (part1, time1) = get_time(|| self.event.part1());
        let (part2, time2) = get_time(|| self.event.part2());

        println!("Solution for day {}", self.day);
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
            "part 2: {} in {}\n",
            part2.fg_rgb::<100, 252, 218>(),
            format_duration(time2).fg_rgb::<100, 252, 218>()
        );
    }
}

pub fn get_solution(main_file: &str, day: u8) {
    let filename = format!("src/day{}/{}.txt", day, main_file);
    let content = &fs::read_to_string(&filename)
        .with_context(|| format!("could not read {} file for day {}", main_file, day))
        .unwrap();

    let solution = match day {
        1 => Solution::new::<day1::SonarSweep>(content),
        2 => Solution::new::<day2::Dive>(content),
        3 => Solution::new::<day3::BinaryDiagnostic>(content),
        4 => Solution::new::<day4::GiantSquid>(content),
        5 => Solution::new::<day5::HydrothermalVenture>(content),
        6 => Solution::new::<day6::Lanternfish>(content),
        7 => Solution::new::<day7::Whales>(content),
        8 => Solution::new::<day8::SevenSegment>(content),
        9 => Solution::new::<day9::SmokeBasin>(content),
        _ => unreachable!(),
    };

    solution.get_result();
}

fn get_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let time = start.elapsed();

    (result, time)
}

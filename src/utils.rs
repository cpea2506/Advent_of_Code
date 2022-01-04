use humantime::format_duration;
use owo_colors::OwoColorize;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
    time::{Duration, Instant},
};

pub trait Avent {
    fn new(data: Vec<String>) -> Self
    where
        Self: Sized;
    fn part1(&self) -> u32;
    fn part2(&self) -> u32;
}

pub struct Solution {
    event: Box<dyn Avent>,
    time: Duration,
}

impl Solution {
    pub fn new<Event: Avent + 'static>(content: Vec<String>) -> Self {
        let (event, time) = get_time(|| Event::new(content));

        Solution {
            event: Box::new(event),
            time,
        }
    }

    pub fn get_result(&self, day: u32) {
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

pub fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Vec<String>> {
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines().flatten().collect())
}

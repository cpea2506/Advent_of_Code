use crate::utils;

pub struct SonarSweep {
    data: Vec<u32>,
}

impl utils::Avent for SonarSweep {
    fn new(data: Vec<String>) -> Self {
        SonarSweep {
            data: data.iter().filter_map(|l| l.parse().ok()).collect(),
        }
    }

    fn part1(&self) -> u32 {
        self.data
            .iter()
            .zip(self.data.iter().skip(1))
            .filter(|(a, b)| a < b)
            .count() as u32
    }

    fn part2(&self) -> u32 {
        self.data
            .windows(3)
            .zip(self.data.windows(3).skip(1))
            .filter(|(a, b)| a.iter().sum::<u32>() < b.iter().sum())
            .count() as u32
    }
}

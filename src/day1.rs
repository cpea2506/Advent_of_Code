use crate::utils;

pub struct SonarSweep {
    data: Vec<usize>,
}

impl utils::Avent for SonarSweep {
    fn new(data: Vec<String>) -> Self {
        SonarSweep {
            data: data.iter().filter_map(|l| l.parse().ok()).collect(),
        }
    }

    fn part1(&self) -> usize {
        self.data
            .iter()
            .zip(self.data.iter().skip(1))
            .filter(|(a, b)| a < b)
            .count()
    }

    fn part2(&self) -> usize {
        self.data
            .windows(3)
            .zip(self.data.windows(3).skip(1))
            .filter(|(a, b)| a.iter().sum::<usize>() < b.iter().sum())
            .count()
    }
}

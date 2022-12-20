pub struct SonarSweep {
    data: Vec<u32>,
}

impl crate::Avent for SonarSweep {
    fn new(data: &str) -> Self {
        SonarSweep {
            data: data.lines().flat_map(|l| l.parse()).collect(),
        }
    }

    fn day() -> u8 {
        1
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
            .filter(|(a, b)| a.iter().sum::<u32>() < b.iter().sum())
            .count()
    }
}

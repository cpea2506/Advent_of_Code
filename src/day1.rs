pub struct SonarSweep {
    data: Vec<usize>,
}

impl crate::Avent for SonarSweep {
    fn new(data: Vec<String>) -> Self {
        SonarSweep {
            data: data.into_iter().flat_map(|l| l.parse()).collect(),
        }
    }

    fn part1(&self) -> usize {
        self.data.windows(2).filter(|d| d[0] < d[1]).count()
    }

    fn part2(&self) -> usize {
        self.data
            .windows(3)
            .zip(self.data.windows(3).skip(1))
            .filter(|(a, b)| a.iter().sum::<usize>() < b.iter().sum())
            .count()
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

pub struct SonarSweep {
    data: Vec<usize>,
}

impl SonarSweep {
    pub fn new(path: &str) -> Result<Self> {
        let lines = read_line(path)?;

        Ok(SonarSweep {
            data: lines.flat_map(|l| l.unwrap().parse()).collect(),
        })
    }

    pub fn part1(&self) -> usize {
        self.data.windows(2).filter(|d| d[0] < d[1]).count()
    }

    pub fn part2(&self) -> usize {
        self.data
            .windows(3)
            .zip(self.data.windows(3).skip(1))
            .filter(|(a, b)| a.iter().sum::<usize>() < b.iter().sum())
            .count()
    }
}

fn read_line<P: AsRef<Path>>(path: P) -> Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}

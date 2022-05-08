type Line = Vec<char>;

pub struct SyntaxScoring {
    chunk_lines: Vec<Line>,
}

impl crate::Avent for SyntaxScoring {
    fn new(data: &str) -> Self {
        let chunk_lines = data
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();

        SyntaxScoring { chunk_lines }
    }

    fn day() -> u8 {
        10
    }

    fn part1(&self) -> usize {
        0
    }

    fn part2(&self) -> usize {
        0
    }
}

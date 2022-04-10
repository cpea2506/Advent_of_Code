pub struct Entry {
    #[allow(unused)]
    patterns: String,
    output: String,
}

pub struct SevenSegment {
    entries: Vec<Entry>,
}

impl SevenSegment {
    fn is_unique(s: &str) -> bool {
        matches!(s.len(), 2 | 3 | 4 | 7)
    }

    fn decode(_patterns: &str) -> usize {
        0
    }
}

impl crate::Avent for SevenSegment {
    fn new(data: &str) -> Self {
        let entries = data
            .lines()
            .map(|l| {
                let mut iter = l.split(" | ").map(|s| s.to_string());
                let patterns = iter.next().unwrap();
                let output = iter.next().unwrap();

                Entry { patterns, output }
            })
            .collect();

        SevenSegment { entries }
    }

    fn day() -> u8 {
        8
    }

    fn part1(&self) -> usize {
        self.entries
            .iter()
            .flat_map(|e| e.output.split_whitespace())
            .filter(|s| Self::is_unique(s))
            .count()
    }

    fn part2(&self) -> usize {
        let _ = self
            .entries
            .iter()
            .map(|e| Self::decode(&e.output))
            .sum::<usize>();

        0
    }
}

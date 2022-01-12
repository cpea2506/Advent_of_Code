pub struct Entry {
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
}

impl crate::Avent for SevenSegment {
    fn new(data: &str) -> Self {
        let digit_instances = data
            .lines()
            .map(|l| {
                let mut entry_iter = l.split(" | ").map(|s| s.to_string());
                let patterns = entry_iter.next().unwrap();
                let output = entry_iter.next().unwrap();

                Entry { patterns, output }
            })
            .collect();

        SevenSegment {
            entries: digit_instances,
        }
    }

    fn part1(&self) -> usize {
        self.entries
            .iter()
            .map(|e| e.output.split_whitespace())
            .flatten()
            .filter(|s| Self::is_unique(s))
            .count()
    }

    fn part2(&self) -> usize {
        0
    }
}

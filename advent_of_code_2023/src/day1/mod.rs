use aho_corasick::AhoCorasick;

pub struct Trebuchet {
    data: Vec<String>,
}

impl crate::Advent for Trebuchet {
    fn new(data: &str) -> Self {
        Self {
            data: data.lines().map(String::from).collect(),
        }
    }

    fn day() -> u8 {
        1
    }

    fn part1(&self) -> usize {
        self.data
            .iter()
            .filter_map(|l| {
                let mut first_digit = None;

                for c in l.chars() {
                    first_digit = c.to_digit(10);

                    if first_digit.is_some() {
                        break;
                    }
                }

                let mut last_digit = None;
                for c in l.chars().rev() {
                    last_digit = c.to_digit(10);

                    if last_digit.is_some() {
                        break;
                    }
                }

                Some(first_digit? as usize * 10 + last_digit? as usize)
            })
            .sum()
    }

    fn part2(&self) -> usize {
        let ac = AhoCorasick::new([
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ])
        .unwrap();

        self.data
            .iter()
            .filter_map(|l| {
                let matched = ac.find_iter(l).collect::<Vec<_>>();
                let first = matched.first()?.pattern().as_usize();
                let last = matched.last()?.pattern().as_usize();

                Some((first / 2 + 1) * 10 + (last / 2 + 1))
            })
            .sum()
    }
}

crate::test!(Trebuchet {
    part1("example.txt") => 142,
    part2("example2.txt") => 281
});

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
            .map(|l| {
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

                first_digit.unwrap() as usize * 10 + last_digit.unwrap() as usize
            })
            .sum()
    }

    fn part2(&self) -> usize {
        let ac = AhoCorasick::new(&[
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ])
        .unwrap();

        self.data
            .iter()
            .map(|l| {
                let matched = ac.find_iter(&l).collect::<Vec<_>>();
                let first = matched.first().unwrap().pattern().as_usize();
                let last = matched.last().unwrap().pattern().as_usize();

                (first / 2 + 1) * 10 + (last / 2 + 1)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Advent;

    #[test]
    fn example_part1() {
        let data = include_str!("example.txt");
        let trebuchet = Trebuchet::new(data);

        assert_eq!(trebuchet.part1(), 142);
    }

    #[test]
    fn example_part2() {
        let data = include_str!("example2.txt");
        let trebuchet = Trebuchet::new(data);

        assert_eq!(trebuchet.part2(), 281);
    }
}

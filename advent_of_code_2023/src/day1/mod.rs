use std::{collections::HashMap, ops::Add};

pub struct Trebuchet {
    data: Vec<Vec<char>>,
}

impl crate::Advent for Trebuchet {
    fn new(data: &str) -> Self {
        Self {
            data: data.lines().map(|l| l.chars().collect()).collect(),
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
                let mut last_digit = None;

                for c in l {
                    first_digit = c.to_digit(10);

                    if first_digit.is_some() {
                        break;
                    }
                }

                for c in l.iter().rev() {
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
        let digit_str = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        self.data
            .iter()
            .map(|l| {
                let mut digits = vec![];

                for (i, &c) in l.iter().enumerate() {
                    if c.is_digit(10) {
                        digits.push(c.to_digit(10).unwrap() as usize);
                    }

                    for (j, &c2) in digit_str.iter().enumerate() {
                        if l[i..].into_iter().collect::<String>().starts_with(c2) {
                            digits.push(j + 1);
                        }
                    }
                }

                digits[0] * 10 + digits[digits.len() - 1]
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

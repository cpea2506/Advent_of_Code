trait Math {
    fn is_even(&self) -> bool;
    fn triangle_sum(s: usize) -> usize {
        (s + 1) * s / 2
    }
}

impl Math for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

pub struct Whales {
    positions: Vec<usize>,
}

impl Whales {
    fn median(&self) -> usize {
        let len = self.positions.len();
        let middle = len / 2;

        if len.is_even() {
            (self.positions[middle - 1] + self.positions[middle]) / 2
        } else {
            self.positions[middle] / 2
        }
    }

    fn mean(&self) -> usize {
        self.positions.iter().sum::<usize>() / self.positions.len()
    }
}

impl crate::Advent for Whales {
    fn new(data: &str) -> Self {
        let mut positions = data
            .split(',')
            .flat_map(|v| v.parse())
            .collect::<Vec<usize>>();
        positions.sort_unstable();

        Whales { positions }
    }

    fn day() -> u8 {
        7
    }

    fn part1(&self) -> usize {
        // the sum of distances to the middle of slice (median) has the lowest fuel
        let median = self.median();

        self.positions.iter().map(|&v| v.abs_diff(median)).sum()
    }

    fn part2(&self) -> usize {
        (self.mean()..)
            .take(2)
            .map(|p| {
                self.positions
                    .iter()
                    .map(|&v| usize::triangle_sum(v.abs_diff(p)))
                    .sum()
            })
            .min()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn triangle_sum_10() {
        let sum = usize::triangle_sum(10);

        assert_eq!(sum, 55)
    }

    #[test]
    fn round_number() {
        let value = 4.6f32.round() as u32;

        assert_eq!(value, 5)
    }

    #[test]
    fn devide_int_with_lost() {
        let lhs = 5;
        let rhs = 3;

        assert_eq!(lhs / rhs, 1)
    }
}

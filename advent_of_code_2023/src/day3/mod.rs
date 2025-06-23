#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    row: usize,
    col: (usize, usize),
}

impl Number {
    fn is_part_number(&self, grid: &Grid) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();

        for line in grid
            .iter()
            .take((self.row + 1).min(rows - 1) + 1)
            .skip(self.row.saturating_sub(1))
        {
            for col in self.col.0.saturating_sub(1)..=(self.col.1 + 1).min(cols - 1) {
                let ch = line.as_bytes()[col] as char;

                if !ch.is_ascii_digit() && ch != '.' {
                    return true;
                }
            }
        }

        false
    }

    fn touches(&self, row: usize, col: usize) -> bool {
        let diff = (self.row as isize - row as isize).abs();

        if diff > 1 {
            return false;
        }

        (self.col.0.saturating_sub(1)..=self.col.1 + 1).contains(&col)
    }
}

type Grid = Vec<String>;

pub struct GearRatios {
    grid: Grid,
    numbers: Vec<Number>,
}

impl crate::Advent for GearRatios {
    fn new(data: &str) -> Self {
        let grid = data.lines().map(str::to_string).collect::<Grid>();
        let mut numbers = vec![];

        for (row, line) in grid.iter().enumerate() {
            let mut col = 0;
            let chars = line.chars().collect::<Vec<char>>();

            while col < chars.len() {
                if chars[col].is_ascii_digit() {
                    let start = col;
                    let mut value = 0;

                    while col < chars.len() && chars[col].is_ascii_digit() {
                        if let Some(digit) = chars[col].to_digit(10) {
                            value = value * 10 + digit;
                            col += 1;
                        }
                    }

                    numbers.push(Number {
                        value,
                        row,
                        col: (start, col - 1),
                    });
                } else {
                    col += 1;
                }
            }
        }

        Self { grid, numbers }
    }

    fn day() -> u8 {
        3
    }

    fn part1(&self) -> usize {
        self.numbers
            .iter()
            .filter_map(|n| n.is_part_number(&self.grid).then_some(n.value as usize))
            .sum()
    }

    fn part2(&self) -> usize {
        let mut sum = 0;

        for (row, line) in self.grid.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != '*' {
                    continue;
                }

                let touching_numbers = self
                    .numbers
                    .iter()
                    .filter(|n| n.touches(row, col))
                    .collect::<Vec<&Number>>();

                if touching_numbers.len() == 2 {
                    sum += touching_numbers[0].value * touching_numbers[1].value;
                }
            }
        }

        sum as usize
    }
}

crate::test!(GearRatios {
    part1("example.txt") => 4361,
    part2("example.txt") => 467835
});

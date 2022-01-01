enum Direction {
    Down,
    Forward,
    Up,
}

struct Command {
    direction: Direction,
    count: usize,
}

pub struct Dive {
    data: Vec<Command>,
}

impl crate::Avent for Dive {
    fn new(data: Vec<String>) -> Dive {
        let data: Vec<Command> = data
            .iter()
            .map(|d| {
                let mut cmd_iter = d.split_whitespace();

                let direction = match cmd_iter.next().unwrap() {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    _ => Direction::Down,
                };
                let count = cmd_iter.next().unwrap().parse().unwrap();

                Command { direction, count }
            })
            .collect();

        Dive { data }
    }

    fn part1(&self) -> usize {
        let mut horziontal = 0;
        let mut depth = 0;

        self.data.iter().for_each(|c| match c.direction {
            Direction::Forward => horziontal += c.count,
            Direction::Down => depth += c.count,
            Direction::Up => depth -= c.count,
        });

        horziontal * depth
    }

    fn part2(&self) -> usize {
        let mut horziontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        self.data.iter().for_each(|c| match c.direction {
            Direction::Forward => {
                horziontal += c.count;
                depth += aim * c.count;
            }
            Direction::Down => aim += c.count,
            Direction::Up => aim -= c.count,
        });

        horziontal * depth
    }
}

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
    commands: Vec<Command>,
}

impl crate::Avent for Dive {
    fn new(data: &str) -> Dive {
        let data: Vec<Command> = data
            .lines()
            .map(|d| {
                let mut cmd_iter = d.split_whitespace();
                let direction = match cmd_iter.next().unwrap() {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    _ => Direction::Down,
                };
                let count = cmd_iter.next().unwrap().parse::<usize>().unwrap();

                Command { direction, count }
            })
            .collect();

        Dive { commands: data }
    }

    fn part1(&self) -> usize {
        let mut horziontal: usize = 0;
        let mut depth: usize = 0;

        self.commands.iter().for_each(|c| match c.direction {
            Direction::Forward => horziontal += c.count,
            Direction::Down => depth += c.count,
            Direction::Up => depth -= c.count,
        });

        (horziontal * depth) as usize
    }

    fn part2(&self) -> usize {
        let mut horziontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        self.commands.iter().for_each(|c| match c.direction {
            Direction::Forward => {
                horziontal += c.count;
                depth += aim * c.count;
            }
            Direction::Down => aim += c.count,
            Direction::Up => aim -= c.count,
        });

        (horziontal * depth) as usize
    }
}

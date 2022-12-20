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
        let commands: Vec<Command> = data
            .lines()
            .map(|d| {
                let cmd = d.split_whitespace().collect::<Vec<&str>>();
                let direction = match cmd[0] {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    _ => Direction::Down,
                };
                let count = cmd[1].parse().unwrap();

                Command { direction, count }
            })
            .collect();

        Dive { commands }
    }

    fn day() -> u8 {
        2
    }

    fn part1(&self) -> usize {
        let mut horziontal = 0;
        let mut depth = 0;

        self.commands.iter().for_each(|c| match c.direction {
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

        self.commands.iter().for_each(|c| match c.direction {
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

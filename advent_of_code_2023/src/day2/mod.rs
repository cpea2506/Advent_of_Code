use std::{collections::HashMap, str::FromStr, sync::LazyLock};

static CUBE_LIMIT: LazyLock<HashMap<Color, u8>> =
    LazyLock::new(|| HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]));

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

type SubSet = HashMap<Color, Vec<u8>>;

#[derive(Debug)]
struct Game {
    id: u8,
    subset: SubSet,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, subset_str) = s
            .split_once(':')
            .and_then(|(g, s)| {
                let id = g.strip_prefix("Game ")?.parse().ok()?;

                Some((id, s))
            })
            .ok_or(())?;

        let mut subset = SubSet::new();

        for s in subset_str.split(';') {
            for e in s.split(',') {
                let mut iter = e.split_whitespace();
                let count = iter.next().ok_or(())?.parse().map_err(|_| ())?;
                let color = iter.next().ok_or(())?.parse()?;

                subset.entry(color).or_default().push(count)
            }
        }

        Ok(Self { id, subset })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.subset
            .iter()
            .all(|(color, count)| count.iter().all(|&x| x <= CUBE_LIMIT[color]))
    }

    fn power(&self) -> u16 {
        self.subset
            .values()
            .filter_map(|count| count.iter().max())
            .map(|&x| x as u16)
            .product()
    }
}

pub struct Conundrum {
    games: Vec<Game>,
}

impl crate::Advent for Conundrum {
    fn new(data: &str) -> Self {
        Self {
            games: data
                .lines()
                .filter_map(|line| line.parse().ok())
                .collect::<Vec<Game>>(),
        }
    }

    fn day() -> u8 {
        2
    }

    fn part1(&self) -> usize {
        self.games
            .iter()
            .filter_map(|g| g.is_valid().then_some(g.id as usize))
            .sum()
    }

    fn part2(&self) -> usize {
        self.games.iter().map(|g| g.power() as usize).sum()
    }
}

crate::test!(Conundrum {
    part1("example.txt") => 8,
    part2("example.txt") => 2286,
});

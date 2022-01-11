use std::cmp::Ordering;

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(coord: &str) -> Self {
        let mut coord_iter = coord.split(',').filter_map(|c| c.parse::<usize>().ok());
        let x = coord_iter.next().unwrap();
        let y = coord_iter.next().unwrap();

        Coordinate { x, y }
    }

    fn is_straight_with(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }
}

pub struct Line {
    start: Coordinate,
    end: Coordinate,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        !self.start.is_straight_with(&self.end)
    }
}

pub struct HydrothermalVenture {
    lines: Vec<Line>,
}

impl crate::Avent for HydrothermalVenture {
    fn new(data: &str) -> Self {
        let lines = data
            .lines()
            .map(|l| {
                let mut coords_iter = l.split(" -> ").map(Coordinate::new);
                let start = coords_iter.next().unwrap();
                let end = coords_iter.next().unwrap();

                Line { start, end }
            })
            .collect();

        HydrothermalVenture { lines }
    }

    fn part1(&self) -> usize {
        count_overlap(self.lines.iter().filter(|l| !l.is_diagonal()))
    }

    fn part2(&self) -> usize {
        count_overlap(self.lines.iter())
    }
}

fn count_overlap<'a>(iter: impl Iterator<Item = &'a Line>) -> usize {
    let width = 1000;
    let mut map = vec![0u8; width * width];
    let middle_points = |a: usize, b: usize| match a.cmp(&b) {
        Ordering::Less => a + 1,
        Ordering::Greater => a - 1,
        _ => a,
    };

    iter.for_each(|Line { start, end }| {
        let Coordinate { mut x, mut y } = start;

        map[x + y * width] += 1;

        while x != end.x || y != end.y {
            x = middle_points(x, end.x);
            y = middle_points(y, end.y);

            map[x + y * width] += 1;
        }
    });

    map.iter().filter(|&&v| v > 1).count()
}

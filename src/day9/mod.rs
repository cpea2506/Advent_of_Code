pub struct SmokeBasin {
    heightmap: Vec<Vec<usize>>,
}

impl crate::Avent for SmokeBasin {
    fn new(data: &str) -> Self {
        let heightmap = data
            .lines()
            .map(|l| l.split("").flat_map(|v| v.parse()).collect())
            .collect();

        SmokeBasin { heightmap }
    }

    fn day() -> u8 {
        9
    }

    fn part1(&self) -> usize {
        const ADJACENT: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut sum: usize = 0;
        let width = self.heightmap.get(0).unwrap().len();
        let len = self.heightmap.len();

        for (i, map) in self.heightmap.iter().enumerate() {
            for (j, &height) in map.iter().enumerate() {
                if ADJACENT
                    .iter()
                    .map(|(x, y)| (i as isize + x, j as isize + y))
                    .filter(|&(x, y)| x >= 0 && x < len as isize && y >= 0 && y < width as isize)
                    .all(|(x, y)| height < self.heightmap[x as usize][y as usize])
                {
                    sum += height + 1
                }
            }
        }

        sum
    }

    fn part2(&self) -> usize {
        0
    }
}

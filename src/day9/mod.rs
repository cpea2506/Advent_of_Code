pub struct SmokeBasin {
    heightmap: Vec<Vec<u8>>,
}

impl crate::Avent for SmokeBasin {
    fn new(data: &str) -> Self {
        let heightmap = data
            .lines()
            .map(|l| l.split("").filter_map(|v| v.parse::<u8>().ok()).collect())
            .collect();

        SmokeBasin { heightmap }
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
                    .filter(|(x, y)| {
                        let new_x = i as isize + x;
                        let new_y = j as isize + y;

                        new_x >= 0 && new_x < len as isize && new_y >= 0 && new_y < width as isize
                    })
                    .all(|&(x, y)| {
                        let new_x = i as isize + x;
                        let new_y = j as isize + y;

                        height < self.heightmap[new_x as usize][new_y as usize]
                    })
                {
                    sum += (height + 1) as usize
                }
            }
        }

        sum
    }

    fn part2(&self) -> usize {
        0
    }
}

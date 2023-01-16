pub struct Lanternfish {
    internal_timer: Vec<u8>,
}

impl Lanternfish {
    fn count(&self, day: usize) -> usize {
        let mut days = [0; 9];

        for &timer in &self.internal_timer {
            days[timer as usize] += 1;
        }

        (0..day).for_each(|_| {
            let new_fishes_count = days[0];

            (1..=8).for_each(|i| days[i - 1] = days[i]);

            days[6] += new_fishes_count;
            days[8] = new_fishes_count;
        });

        days.iter().sum()
    }
}

impl crate::Advent for Lanternfish {
    fn new(data: &str) -> Self {
        let internal_timer = data.split(',').flat_map(|v| v.parse()).collect();

        Lanternfish { internal_timer }
    }

    fn day() -> u8 {
        6
    }

    fn part1(&self) -> usize {
        self.count(80)
    }

    fn part2(&self) -> usize {
        self.count(256)
    }
}

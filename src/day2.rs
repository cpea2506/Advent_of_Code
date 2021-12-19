struct Data {
    direction: String,
    count: usize,
}

pub struct Dive {
    data: Vec<Data>,
}

impl crate::Avent for Dive {
    fn new(data: Vec<String>) -> Dive {
        let data: Vec<Data> = data
            .into_iter()
            .map(|d| {
                let s: Vec<&str> = d.split_whitespace().collect();

                Data {
                    direction: s[0].to_string(),
                    count: s[1].parse().unwrap(),
                }
            })
            .collect();

        Dive { data }
    }

    fn part1(&self) -> usize {
        let mut horziontal = 0;
        let mut depth = 0;

        self.data.iter().for_each(|d| match d.direction.as_str() {
            "forward" => horziontal += d.count,
            "down" => depth += d.count,
            _ => depth -= d.count,
        });

        horziontal * depth
    }

    fn part2(&self) -> usize {
        let mut horziontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        self.data.iter().for_each(|d| match d.direction.as_str() {
            "forward" => {
                horziontal += d.count;
                depth += aim * d.count;
            }
            "down" => aim += d.count,
            _ => aim -= d.count,
        });

        horziontal * depth
    }
}

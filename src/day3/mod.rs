trait Binary {
    fn add_bit(self, bit: bool) -> usize;
}

impl Binary for usize {
    fn add_bit(self, bit: bool) -> usize {
        self << 1 | bit as usize
    }
}

type Bin = Vec<u8>;

pub struct BinaryDiagnostic {
    data: Vec<Bin>,
}

impl crate::Avent for BinaryDiagnostic {
    fn new(data: &str) -> Self {
        BinaryDiagnostic {
            data: data
                .lines()
                .map(|l| l.bytes().map(|b| b - 48).collect::<Bin>())
                .collect(),
        }
    }

    fn day() -> u8 {
        3
    }

    fn part1(&self) -> usize {
        let mut iter = self.data.iter();

        let mut final_bin = iter
            .next()
            .unwrap()
            .iter()
            .map(|&v| v as usize)
            .collect::<Vec<usize>>();

        iter.flat_map(|v| v.iter().enumerate()).for_each(|(i, &v)| {
            if v > 0 {
                final_bin[i] += 1
            }
        });

        let mut gamma = 0;
        let mut epsilon = 0;
        let half_data_len = self.data.len() >> 1;

        final_bin.iter().for_each(|&v| {
            let bit = v / half_data_len;

            gamma = gamma.add_bit(bit > 0);
            epsilon = epsilon.add_bit(bit < 1);
        });

        gamma * epsilon
    }

    fn part2(&self) -> usize {
        let bit_criteria = |common: bool| {
            let mut indexes = (0..self.data.len()).collect::<Vec<usize>>();
            let mut current_index = 0;

            while indexes.len() > 1 {
                let mut sum = 0;

                indexes.iter().for_each(|&i| {
                    if self.data[i][current_index] > 0 {
                        sum += 1
                    }
                });

                let check_bit = sum / (indexes.len() - (indexes.len() >> 1));
                let bit = ((check_bit > 0) ^ !common) as u8;

                indexes.retain(|&i| self.data[i][current_index] == bit);

                current_index += 1;
            }

            self.data[indexes[0]].clone()
        };

        let add_bit = |acc: usize, &b: &u8| acc.add_bit(b > 0);
        let oxigen = bit_criteria(true).iter().fold(0, add_bit);
        let co2 = bit_criteria(false).iter().fold(0, add_bit);

        oxigen * co2
    }
}

use crate::utils;

pub struct BinaryDiagnostic {
    data: Vec<Vec<u8>>,
}

impl utils::Avent for BinaryDiagnostic {
    fn new(data: Vec<String>) -> Self {
        BinaryDiagnostic {
            data: data
                .iter()
                .map(|l| l.bytes().map(|b| b - 48).collect())
                .collect(),
        }
    }

    fn part1(&self) -> usize {
        let mut sum_col_vec: Vec<usize> = vec![0; self.data[0].len()];

        for bin in self.data.iter() {
            for (i, &bit) in bin.iter().enumerate() {
                sum_col_vec[i] += bit as usize;
            }
        }

        let mut gamma: usize = 0;
        let mut epsilon: usize = 0;

        let half_len = self.data.len() >> 1;
        for (i, sum) in sum_col_vec.iter().rev().enumerate() {
            let pow2 = 1 << i;
            let bit = sum / half_len;

            gamma |= bit * pow2;
            epsilon |= (1 ^ bit) * pow2;
        }

        gamma * epsilon
    }

    fn part2(&self) -> usize {
        0
    }
}

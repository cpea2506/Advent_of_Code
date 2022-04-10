type Board = [u8; 25];

pub struct GiantSquid {
    draw_nums: Vec<u8>,
    boards: Vec<Board>,
}

impl crate::Avent for GiantSquid {
    fn new(data: &str) -> Self {
        let mut iter = data.lines();

        let draw_nums = iter
            .next()
            .unwrap()
            .split(',')
            .flat_map(|c| c.parse())
            .collect();

        let mut boards = Vec::new();

        while iter.next().is_some() {
            let board = iter
                .by_ref()
                .take(5)
                .flat_map(|s| s.split_whitespace())
                .flat_map(|c| c.parse())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();

            boards.push(board);
        }

        GiantSquid { draw_nums, boards }
    }

    fn day() -> u8 {
        4
    }

    fn part1(&self) -> usize {
        let (board, last_draw_index) = self
            .boards
            .iter()
            .map(|&b| (b, last_draw_index(b, &self.draw_nums)))
            .min_by_key(|t| t.1)
            .unwrap();

        winning_score(board, &self.draw_nums, last_draw_index)
    }

    fn part2(&self) -> usize {
        let (board, last_draw_index) = self
            .boards
            .iter()
            .map(|&b| (b, last_draw_index(b, &self.draw_nums)))
            .max_by_key(|t| t.1)
            .unwrap();

        winning_score(board, &self.draw_nums, last_draw_index)
    }
}

fn winning_score(board: Board, draw_nums: &[u8], last_draw_index: Option<usize>) -> usize {
    if let Some(index) = last_draw_index {
        let draw_numbers = draw_nums.get(..=index).unwrap();
        let unmarked_sum = board
            .iter()
            .filter(|v| !draw_numbers.contains(v))
            .sum::<u8>();

        (draw_numbers[index] * unmarked_sum) as usize
    } else {
        0
    }
}

fn last_draw_index(board: Board, draw_numers: &[u8]) -> Option<usize> {
    let mut rows: [u8; 5] = [0; 5];
    let mut cols: [u8; 5] = [0; 5];

    for (i, number) in draw_numers.iter().enumerate() {
        if let Some(position) = board.iter().position(|v| v == number) {
            let row_index = position / 5;
            let col_index = position % 5;

            rows[row_index] += 1;
            cols[col_index] += 1;

            if rows[row_index] == 5 || cols[col_index] == 5 {
                return Some(i);
            }
        };
    }

    None
}

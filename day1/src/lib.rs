use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

pub fn count_depth_increases<F>(filename: F) -> Result<usize>
where
    F: AsRef<Path>,
{
    let file = File::open(filename)?;

    let count = BufReader::new(file)
        .lines()
        .flat_map(|l| l.unwrap().parse::<usize>())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|d| d[0] < d[1])
        .count();

    Ok(count)
}

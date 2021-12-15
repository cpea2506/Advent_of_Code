use day1::count_depth_increases;

fn main() {
    let count = count_depth_increases("input.txt").expect("Error reading file!");
    println!("{}", count);
}

mod sonar_sweep;

use sonar_sweep::SonarSweep;

fn main() {
    let sonar_sweep = SonarSweep::new("input.txt").expect("Error reading file!");

    let count = sonar_sweep.part1();
    println!("part 1: {}", count);

    let window_count = sonar_sweep.part2();
    println!("part 2: {}", window_count);
}

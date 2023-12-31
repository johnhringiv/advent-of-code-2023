use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;


fn main() {
    let days = [day01::day01, day02::day02, day03::day03, day04::day04, day05::day05, day06::day06, day07::day07, day08::day08];
    println!(
        "{0: <3} | {1: <10} | {2: <14} | {3: <10}",
        "Day", "Part 1", "Part 2", "μs"
    );
    for (idx, day) in days.iter().enumerate() {
        let now = Instant::now();
        let (p1, p2) = day();
        let micros = now.elapsed().as_micros();
        println!("{0:<3} | {1:<10} | {2:<14} | {3:<10}", format!{"{:02}", idx+1}, p1, p2, micros)
    }
}
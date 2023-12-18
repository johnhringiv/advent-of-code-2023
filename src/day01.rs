use lazy_static::lazy_static;
use regex::Regex;
use advent_2023::load_data;

fn get_first_digit<T: Iterator<Item = char>>(chars: T) -> usize {
    for c in chars {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap() as usize
        }
    }
    panic!("Invalid Input")
}

fn process_match(digit_str: &str) -> usize {
    match digit_str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit_str.parse::<usize>().unwrap()
    }
}

fn get_digit_from_window<'a, T: Iterator<Item=&'a [u8]>>(windows: T) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    }
    for win in windows {
        let window_str = String::from_utf8_lossy(win).to_string();
        match RE.find(&window_str) {
            Some(val) => return process_match(&val.as_str()),
            None => continue
        }
    }
    panic!("Invalid input")
}

fn run_iterative(data: &str) -> (usize, usize) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    for line in data.lines() {
        ans1 += get_first_digit(line.chars()) * 10 + get_first_digit(line.chars().rev());

        let line_padded = line.to_owned() + "    ";
        let inter = line_padded.as_bytes();
        let windows = inter.windows(5);
        ans2 += get_digit_from_window(windows.clone()) * 10 + get_digit_from_window(windows.rev());
    }
    (ans1, ans2)
}

pub fn day01() -> (usize, usize) {
    let data = load_data(1);
    run_iterative(&data)
}

#[cfg(test)]
mod tests {
    use crate::day01::*;

    #[test]
    fn p1_example() {
        let data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let (p1, _) = run_iterative(&data);
        assert_eq!(p1, 142)
    }

    #[test]
    fn puzzle_ans() {
        assert_eq!(day01(), (53651, 53894))
    }
}

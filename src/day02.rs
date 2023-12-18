use std::cmp::max;
use regex::Regex;
use advent_2023::load_data;

fn cube_conundrum(data: &str) -> (usize, usize) {
    let (mut part1, mut part2) = (0, 0);
    let re: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    for (mut game_id, line) in data.lines().enumerate() {
        game_id += 1;

        let (mut red, mut green, mut blue) = (0, 0, 0);

        for draw in re.captures_iter(line) {
            match (draw.get(1), draw.get(2)) {
                (Some(amount_match), Some(color_match)) => {
                    let amount = amount_match.as_str().parse::<usize>().unwrap();
                    match color_match.as_str() {
                        "red" => red = max(red, amount),
                        "green" => green = max(green, amount),
                        "blue" => blue = max(blue, amount),
                        _ => panic!("Invalid Color")
                    };
                }
                _ => panic!("Invalid Input")
            }
        }
        if (red <= 12) & (green <= 13) & (blue <= 14) {
            part1 += game_id
        }
        part2 += red * green * blue
    }
    (part1, part2)
}

pub fn day02() -> (usize, usize) {
    // consider more efficient IO https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    let data = load_data(2);
    cube_conundrum(&data)
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    #[test]
    fn example_input() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(cube_conundrum(data), (8, 2286))
    }

    #[test]
    fn puzzle_ans() {
        assert_eq!(day02(), (2006, 84911))
    }
}
use advent_2023::load_data;

fn scratchcards(data: &str) -> (usize, usize) {
    let mut split_idx: Option<usize> = None;
    let mut multipler = vec![1; data.lines().count()];
    let mut points = vec![0; data.lines().count()];
    for (idx, line) in data.lines().enumerate() {
        let mut card: Vec<&str> = line.split_once(':').unwrap().1.split_whitespace().collect();
        if split_idx.is_none() { split_idx = card.iter().position(|&x| x == "|"); }
        card.remove(split_idx.unwrap());
        let mut winning: Vec<_> = card.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let mine = winning.split_off(split_idx.unwrap());
        let matches = mine.iter().map(|x| winning.contains(x) as usize).sum::<usize>(); // wins copy of next N
        for game in idx+1..idx+1+matches {
            multipler[game] += multipler[idx];
        }
        points[idx] = if matches == 0 {0} else {2_usize.pow((matches - 1) as u32)};
    }
    (points.iter().sum(), multipler.iter().sum())
}

pub fn day04() -> (usize, usize) {
    let data = load_data(4);
    scratchcards(&data)
}

#[cfg(test)]
mod tests {
    use crate::day04::*;

    #[test]
    fn example_input() {
        let data =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let (p1, p2) = scratchcards(&data);
        assert_eq!(scratchcards(&data), (13, 30));
    }

    #[test]
    fn test_ans() {
        assert_eq!(day04(), (21821, 5539496))
    }
}
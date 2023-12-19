use std::iter::zip;
use std::cmp::Ordering;
use advent_2023::load_data;

fn find_num_ways(time: usize, distance: usize) -> usize {
    fn get_traveled(point: usize, time: usize) -> usize {
        (time - point) * point
    }

    let (mut l , mut r) = (0, time/2);
    let mut m = (l + r) / 2;
    while l <= r {
        m = (l + r) / 2;
        match get_traveled(m, time).cmp(&distance) {
            Ordering::Less => l = m + 1,
            Ordering::Greater => r = m - 1,
            _ => break
        }
    }
    if get_traveled(m, time) <= distance { m += 1 } // ending on left split may be less than target
    let points_to_greatest = time / 2 - m;
    let num_greatest = if get_traveled(time / 2, time) == get_traveled(time / 2 + 1, time) {2} else {1};
    points_to_greatest * 2 + num_greatest
}


fn wait_for_it(data: &str) -> (usize, usize) {
    let x: Vec<Vec<usize>> = data.lines().map(|x| x.split_once(':').unwrap().1.split_whitespace().map(|y| y.parse::<usize>().unwrap()).collect()).collect();
    let ans: Vec<usize> = zip(&x[0], &x[1]).map(|(t, d)| find_num_ways(*t, *d)).collect();

    let p2_data: Vec<usize> = x.iter().map(|v| v.iter().rev()
        .fold(0_usize, |res, val| res + *val * 10_usize.pow(res.checked_ilog10().unwrap_or(0) + 1))).map(|x| x / 10).collect();

    (ans.iter().product::<usize>(), find_num_ways(p2_data[0], p2_data[1]))
}

pub fn day06() -> (usize, usize) {
    let data = load_data(6);
    wait_for_it(&data)
}

#[cfg(test)]
mod tests {
    use crate::day06::*;

    #[test]
    fn example_input() {
        let data =
            "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(wait_for_it(&data), (288, 71503))
    }

    #[test]
    fn test_search() {
        assert_eq!(find_num_ways(7, 9), 4);
        assert_eq!(find_num_ways(15, 40), 8);
        assert_eq!(find_num_ways(30, 200), 9);
    }

    #[test]
    fn test_ans() {
        assert_eq!(day06(), (449550, 28360140))
    }
}
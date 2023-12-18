use std::collections::HashMap;
use itertools::Itertools;
use advent_2023::load_data;

fn get_neighbor_labels(pos: usize, array_width: usize, pos_to_label: &HashMap<usize, usize>) -> Vec<&usize> {
    let x_idx = pos / array_width;
    let y_idx = pos % array_width;
    let mut neighbors = vec![];
    // left, right, up, down, ul, ur, ll, lr
    for (x, y) in [(x_idx - 1, y_idx), (x_idx + 1, y_idx), (x_idx, y_idx + 1), (x_idx, y_idx - 1), (x_idx - 1, y_idx + 1), (x_idx + 1, y_idx + 1), (x_idx - 1, y_idx - 1), (x_idx + 1, y_idx - 1)] {
        if (0..array_width).contains(&x) && (y > 0) {
            let neighbor_pos = x * array_width + y;
            neighbors.push(pos_to_label.get(&neighbor_pos));
        }
    }
    neighbors.iter().filter_map(|p| *p).unique().collect::<Vec<&usize>>()
}

fn gear_ratios(data_raw: &str) -> (usize, usize) {
    let array_width = data_raw.split_once('\n').unwrap().0.len();
    let data: Vec<_> = data_raw.replace('\n', "").chars().collect();

    let mut pos_to_label = HashMap::new();
    let mut label_to_value = HashMap::new();
    let mut gear_pos = vec![];
    let mut symbol_pos = vec![];

    fn update_maps(label: &mut usize, num_grp: &mut Vec<usize>, pos_to_label: &mut HashMap<usize, usize>, label_to_value: &mut HashMap<usize, usize>, data: &[char]) {
        if !num_grp.is_empty() {
            pos_to_label.extend(num_grp.iter_mut().map(|p| (*p, *label)));

            label_to_value.insert(*label, num_grp.iter().fold(0, |acc, elem| acc * 10 + data[*elem].to_digit(10).unwrap() as usize));
            num_grp.clear();
            *label += 1;
        }
    }

    let mut label = 0;
    let mut num_grp = vec![];

    let mut start = 0;
    while start != data.len() { // iterate over slices of array_width so we don't have to track rows for numbers
        for (char, pos) in data[start..start + array_width].iter().zip(start..start + array_width) {
            if char.is_ascii_digit() {
                num_grp.push(pos);
            } else {
                if *char == '*' { gear_pos.push(pos) }
                if *char != '.' { symbol_pos.push(pos) }
                update_maps(&mut label, &mut num_grp, &mut pos_to_label, &mut label_to_value, &data);
            }
        }
        update_maps(&mut label, num_grp.as_mut(), &mut pos_to_label, &mut label_to_value, &data);
        start += array_width
    }

    let ans1 = symbol_pos.iter()
        .flat_map(|s| get_neighbor_labels(*s, array_width, &pos_to_label)).unique()
        .map(|l| label_to_value[l]).sum();


    let ans2: usize = gear_pos.iter()
        .map(|gear| get_neighbor_labels(*gear, array_width, &pos_to_label))
        .filter(|x| x.len() == 2).map(|l| label_to_value[&l[0]] * label_to_value[&l[1]]).sum();


    (ans1, ans2)
}


pub fn day03() -> (usize, usize) {
    let data = load_data(3);
    gear_ratios(&data)
}

#[cfg(test)]
mod tests {
    use crate::day03::*;

    #[test]
    fn example_input() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (p1, p2) = gear_ratios(&data);
        assert_eq!(p1, 4361);
        assert_eq!(p2, 467835)
    }

    #[test]
    fn test_ans() {
        assert_eq!(day03(), (549908, 81166799))
    }
}
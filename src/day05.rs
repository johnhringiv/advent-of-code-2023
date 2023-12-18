use std::cmp::min;
use advent_2023::load_data;

fn process_data(data: &str) -> Vec<Vec<usize>> {
    let ret = data.split("\n\n")
        .map(|x| x.split_once(':').unwrap().1.replace('\n', " ").split_whitespace()
            .map(|z| z.parse::<usize>().unwrap()).collect::<Vec<_>>()).collect();
    ret
}
fn part1(sections: &Vec<Vec<usize>>) -> usize {
    let mut target_vals = sections[0].clone().to_owned();
    for section in sections[1..].iter() {
        'target: for target in target_vals.iter_mut() {
            for chunk in section.chunks(3) {
                if let &[dest_start, source_start, map_range] = chunk {
                    // each time we use a vec of present values to build a map, initial vec is seeds
                    if (source_start <= *target) && ((source_start + map_range) >= *target) {
                        *target = dest_start + *target - source_start;
                        continue 'target
                    }
                }
            } // if not matched target remains unchanged
        }
    }
    *target_vals.iter().min().unwrap()
}

fn part2(sections: &Vec<Vec<usize>>) -> usize {
    let mut next_target = sections[0].clone().to_owned();
    for section in sections[1..].iter() {
        let mut target_vals = next_target.clone();
        next_target.clear();

        'target: while let (Some(target_range), Some(target_start)) = (target_vals.pop(), target_vals.pop()) {
            let target_end = target_start + target_range;

            for chunk in section.chunks(3) {
                if let &[dest_start, source_start, map_range] = chunk {
                    let source_end = source_start + map_range;

                    // check if target and source have overlap
                    if (target_start <= source_end) && (source_start <= target_end) {

                        // in case of partial match add the non-overlapped parts back to vec
                        if target_start < source_start {
                            target_vals.push(target_start);
                            target_vals.push(source_start - target_start - 1);
                        }

                        if target_end > source_end {
                            target_vals.push(source_end + 1);
                            target_vals.push(target_end - source_end - 1);
                        }

                        // get the overlap
                        let (offset, overlap_start) = if source_start < target_start {
                            (target_start - source_start, target_start)
                        } else {
                            (0, source_start)
                        };
                        let overlap_range = min(target_end, source_end) - overlap_start;
                        next_target.push(dest_start + offset);
                        next_target.push(overlap_range);
                        continue 'target
                    }
                }
            }
            next_target.push(target_start);
            next_target.push(target_range);
        }
    }
    *next_target.iter().enumerate().filter_map(|(i, v)| if (i % 2) == 0 {Some(v)} else {None}).min().unwrap()
}

// find partial mappings, construct new range, loop until no new matches being found use a stack
pub fn day05() -> (usize, usize) {
    let data = load_data(5);
    let sections = process_data(&data);

    let p1 = part1(&sections);
    let p2 = part2(&sections);
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use crate::day05::*;

    #[test]
    fn example_input() {
        let data =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let sections = process_data(data);
        let p1 = part1(&sections);
        let p2 = part2(&sections);
        assert_eq!(p1, 35);
        assert_eq!(p2, 46)
    }

    #[test]
    fn test_ans() {
        assert_eq!(day05(), (265018614, 63179500))
    }
}
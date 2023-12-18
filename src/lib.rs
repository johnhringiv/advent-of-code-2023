use std::fs::read_to_string;

pub fn load_data(day: usize) -> String {
    if let Ok(data) = read_to_string(format!("data/day{day:02}.txt")) {
        data
    } else { panic!("Missing puzzle input!"); }
}
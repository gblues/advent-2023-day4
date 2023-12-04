use std::collections::HashSet;
use regex::Regex;

fn main() {
    let input = include_str!("input/4.txt");

    let card_pattern = Regex::new(r"(?m)^Card\s+(\d+):\s+([^|]+)\s+\|\s+([^|]+)\s*$").unwrap();
    let mut total: u32 = 0;

    for (_, [card_id, winning_numbers, have_numbers]) in card_pattern.captures_iter(input).map(|c|c.extract::<3>()) {
        let winning_set: HashSet<i32> = HashSet::from_iter(winning_numbers.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>());
        let have_set: HashSet<i32> = HashSet::from_iter(have_numbers.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>());
        let count: u32 = winning_set.intersection(&have_set).count() as u32;
        if count > 0 {
            let points:u32 = 2_i32.pow(count - 1) as u32;
            println!("card {card_id} has {count} matches worth {points} points");
            total += points;
        }
    }
    println!("Total points: {total}");
}

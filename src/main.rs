use std::collections::HashSet;
use regex::Regex;

fn main() {
    let input = include_str!("input/4.txt");

    let card_pattern = Regex::new(r"(?m)^Card\s+(\d+):\s+([^|]+)\s+\|\s+([^|]+)\s*$").unwrap();
    let total: u32;

    let matches: Vec<u32> = card_pattern.captures_iter(input)
        .map(|c|c.extract::<3>())
        .map(|(_, [card_id, winning_numbers, have_numbers])| {
            let winning_set: HashSet<i32> = HashSet::from_iter(
                winning_numbers.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
            );
            let have_set: HashSet<i32> = HashSet::from_iter(
                have_numbers.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
            );

            let count: u32 = winning_set.intersection(&have_set).count() as u32;
            println!("Card {card_id} has {count} matches");
            return count;
        }).collect::<Vec<_>>();
    let mut card_counts: Vec<u32> = vec![1; matches.len().try_into().unwrap()];
    for i in 0..matches.len() {
        if matches[i] > 0 {
            for _ in 1..= card_counts[i] as usize {
                for card in 1..=matches[i] as usize {
                    card_counts[i + card] += 1;
                }
            }
        }
    }
    total = card_counts.iter().sum();
    println!("Total cards: {}", total);
}

use std::collections::{HashMap, HashSet};

type CardResults = HashMap<String, Vec<u32>>;

fn construct_card_game(cards_str: &str) -> (String, Vec<u32>, HashSet<u32>) {
    if let Some((cardn, card_line)) = cards_str.split_once(':') {
        if let Some((winning_str, own_hand_str)) = card_line.rsplit_once('|') {
            let winning_numbers: Vec<u32> = winning_str
                .split_whitespace()
                .map(|hand_content| hand_content.parse::<u32>().unwrap())
                .collect();

            let own_hand = own_hand_str.split_whitespace().fold(
                HashSet::new(),
                move |mut acc, hand_content| {
                    acc.insert(hand_content.parse::<u32>().unwrap());
                    acc
                },
            );
            (cardn.to_owned(), winning_numbers, own_hand)
        } else {
            ("".to_owned(), Vec::new(), HashSet::new())
        }
    } else {
        ("".to_owned(), Vec::new(), HashSet::new())
    }
}

pub fn find_winning_card_scores(input_cards: &str) -> CardResults {
    let (card, winning_nums, hand) = construct_card_game(input_cards);
    let matching_numbers: Vec<u32> = winning_nums
        .into_iter()
        .filter(|num| hand.contains(num))
        .collect();

    HashMap::from([(card.clone(), matching_numbers)])
}

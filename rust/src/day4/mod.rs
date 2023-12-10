use std::collections::{BTreeMap, HashSet};

type CardResults = BTreeMap<String, HashSet<u32>>;

fn construct_card_game(cards_str: &str) -> (&str, HashSet<u32>, HashSet<u32>) {
    if let Some((cardn, card_line)) = cards_str.split_once(':') {
        if let Some((winning_str, own_hand_str)) = card_line.split_once('|') {
            let winning_numbers: HashSet<u32> = winning_str
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
            (cardn, winning_numbers, own_hand)
        } else {
            ("", HashSet::new(), HashSet::new())
        }
    } else {
        ("", HashSet::new(), HashSet::new())
    }
}

fn find_winning_matches_in_card(card_line: &str) -> (String, HashSet<u32>) {
    let (card, winning_nums, hand) = construct_card_game(card_line);
    let matching_numbers: HashSet<u32> =
        winning_nums
            .intersection(&hand)
            .fold(HashSet::new(), move |mut set, num| {
                set.insert(*num);
                set
            });
    (card.to_owned(), matching_numbers)
}

pub fn find_winning_card_scores(input_cards: &str) -> CardResults {
    input_cards
        .split('\n')
        .map(find_winning_matches_in_card)
        .collect()
}

pub fn calc_score(matching_numbers: &HashSet<u32>) -> u32 {
    if matching_numbers.len() > 0 {
        let base = 2u32;
        let n_matching_numbers = matching_numbers.iter().fold(0, |accu, _| accu + 1);
        base.pow(n_matching_numbers - 1)
    } else {
        0
    }
}

pub fn calc_card_pile_size(card_with_wins: CardResults) -> u32 {
    1
}

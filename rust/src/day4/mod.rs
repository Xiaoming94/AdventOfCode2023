use std::collections::{HashMap, HashSet};

type CardResults = HashMap<String, Vec<u32>>;

fn construct_card_game(cards_str: &str) -> (String, Vec<u32>, HashSet<u32>) {
    ("".to_owned(), Vec::new(), HashSet::new())
} 

pub fn find_winning_card_scores(input_cards: &str) -> CardResults {
    let (card, winning_nums, hand) = construct_card_game(input_cards);
    let matching_numbers: Vec<u32> = winning_nums
        .into_iter()
        .filter(|num| hand.contains(num))
        .collect();
        
    HashMap::from([(card.clone(), matching_numbers)])
}

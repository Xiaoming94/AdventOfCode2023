use std::collections::{BTreeMap, HashSet};


#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CardE {
    Card (u32),
}

impl From<&String> for CardE {
    fn from(card_str: &String) -> Self {
        if let Some(card_num) = card_str.split_whitespace().last() {
            match card_num.parse::<u32>() {
                Ok(n) => CardE::Card(n),
                Err(_) => CardE::Card(0)
            }
        } else {
            CardE::Card(0)
        }
    }
}

impl From<&str> for CardE {
    fn from(card_str: &str) -> Self {
        if let Some(card_num) = card_str.split_whitespace().last() {
            match card_num.parse::<u32>() {
                Ok(n) => CardE::Card(n),
                Err(_) => CardE::Card(0)
            }
        } else {
            CardE::Card(0)
        }
    }
}

impl ToString for CardE {
    fn to_string(&self) -> String {
        if let CardE::Card(n) = self {
            "Card ".to_owned() + n.to_string().as_str()
        } else {
            "Card 0".to_owned()
        }
    }
}

type CardResultsInternal = BTreeMap<CardE, HashSet<u32>>;

type CardResults = BTreeMap<String, HashSet<u32>>;

fn construct_card_game(cards_str: &str) -> (CardE, HashSet<u32>, HashSet<u32>) {
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
            (cardn.into(), winning_numbers, own_hand)
        } else {
            (CardE::Card(0), HashSet::new(), HashSet::new())
        }
    } else {
        (CardE::Card(0), HashSet::new(), HashSet::new())
    }
}

fn find_winning_matches_in_card(card_line: &str) -> (CardE, HashSet<u32>) {
    let (card, winning_nums, hand) = construct_card_game(card_line);
    let matching_numbers: HashSet<u32> =
        winning_nums
            .intersection(&hand)
            .fold(HashSet::new(), move |mut set, num| {
                set.insert(*num);
                set
            });
    (card, matching_numbers)
}

fn find_matching_cards(input_cards: &str) -> CardResultsInternal {
    input_cards
        .split('\n')
        .map(find_winning_matches_in_card)
        .collect()
}

pub fn find_winning_card_scores(input_cards: &str) -> CardResults {
    find_matching_cards(input_cards)
        .into_iter()
        .map(|(card, matching)| (card.to_string(), matching))
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

pub fn calc_card_pile_size(input_cards: &str) -> u32 {
    1
}

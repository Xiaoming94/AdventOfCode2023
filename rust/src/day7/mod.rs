use std::{
    cmp::Ordering,
    collections::{btree_map::Values, BTreeMap, HashMap},
    iter::zip,
};

#[derive(Debug, Eq, Hash, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Number(u32),
}

impl Card {
    pub fn get_value(&self) -> u32 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Ten => 10,
            Self::Number(val) => *val,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else if self.get_value() > other.get_value() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl From<char> for Card {
    fn from(cardc: char) -> Self {
        match cardc {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            n if n.is_digit(10) => Self::Number(n.to_digit(10).unwrap()),
            _ => panic!("Error: following character is not a card {}", cardc),
        }
    }
}

#[derive(PartialEq, Eq)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn priority(&self) -> u32 {
        match self {
            Self::Five => 6,
            Self::Four => 5,
            Self::FullHouse => 4,
            Self::Three => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else if self.priority() > other.priority() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type HandCards = [Card; 5];
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: HandCards,
}

impl Hand {
    pub fn get_hand_type(&self) -> HandType {
        let card_counter: HashMap<Card, u32> =
            self.hand
                .iter()
                .fold(HashMap::new(), move |mut card_counter, card| {
                    card_counter
                        .entry(*card)
                        .and_modify(|count| {
                            *count += 1;
                        })
                        .or_insert(1);
                    card_counter
                });

        if card_counter.len() == 1 {
            HandType::Five
        } else if card_counter.len() == 5 {
            HandType::HighCard
        } else if card_counter.values().any(|c| *c == 4) {
            HandType::Four
        } else if card_counter.values().any(|c| *c == 3) {
            if card_counter.values().any(|c| *c == 2) {
                HandType::FullHouse
            } else {
                HandType::Three
            }
        } else if card_counter.len() == 3 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    }
}

impl From<&str> for Hand {
    fn from(hand_str: &str) -> Self {
        let cards: Vec<Card> = hand_str.chars().into_iter().map(Card::from).collect();
        assert!(cards.len() == 5);
        Hand {
            hand: cards.try_into().unwrap_or_else(|s| {
                panic!(
                    "fatal error in system, input should have been filtered: {:?}",
                    s
                )
            }),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if !self.get_hand_type().eq(&other.get_hand_type()) {
            self.get_hand_type().cmp(&other.get_hand_type())
        } else {
            for (card1, card2) in zip(&self.hand, &other.hand) {
                if !card1.eq(card2) {
                    return card1.cmp(card2);
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type BidType = u32;

fn parse_hand_bid(hand_bid_str: &str) -> (Hand, BidType) {
    if let Some((hand_data, bid_str)) = hand_bid_str.split_once(' ') {
        let hand = Hand::from(hand_data);
        let bid = bid_str.parse::<u32>().unwrap();
        (hand, bid)
    } else {
        panic!("Faulty formatted hand bid input: {:?}", hand_bid_str);
    }
}

fn calculate_bid_returns(bids: Vec<&u32>) -> Vec<u32> {
    zip(1..=bids.len(), bids)
        .into_iter()
        .map(|(rank, bid)| {
            let rank_u32: u32 = rank.try_into().unwrap();
            *bid * rank_u32
        })
        .collect()
}

pub fn compute_hands_bid_value(hands_bid_data: &str) -> u32 {
    let hands_to_bids = hands_bid_data.split("\n").map(parse_hand_bid).fold(
        BTreeMap::new(),
        move |mut ranked_handbids_map, (hand, bid)| {
            ranked_handbids_map.insert(hand, bid);
            ranked_handbids_map
        },
    );
    println!("{:?}", hands_to_bids);

    let bid_returns = calculate_bid_returns(hands_to_bids.values().collect());
    return bid_returns.iter().sum();
}

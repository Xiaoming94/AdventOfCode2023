use std::cmp::Ordering;

#[derive(Debug, Eq)]
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

struct Hand {
    hand: [Card; 5],
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            hand: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
        }
    }
}

impl From<&str> for Hand {
    fn from(hand_str: &str) -> Self {
        assert!(hand_str.len() == 5);
        let mut cards: Vec<Card> = hand_str.chars().into_iter().map(Card::from).collect();

        cards.sort();
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

type BidType = u32;

fn parse_hand_bid(hand_bid_str: &str) -> (Hand, BidType) {
    if let Some((hand_data, bid_str)) = hand_bid_str.split_once(" ") {
        let hand = Hand::from(hand_data);
        let bid = bid_str.parse::<u32>().unwrap();
        (hand, bid)
    } else {
        panic!("Faulty formatted hand bid input: {:?}", hand_bid_str);
    }
}

pub fn compute_hands_bid_value(hands_bid_data: &str) -> u32 {
    let (hand, bid) = parse_hand_bid(hands_bid_data);
    bid
}

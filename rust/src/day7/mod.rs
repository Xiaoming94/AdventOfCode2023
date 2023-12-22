use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
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

#[derive(Debug, PartialEq, Eq)]
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
    jacks_as_joker: bool,
}

impl Hand {
    pub fn enable_joker(&mut self) {
        self.jacks_as_joker = true;
    }

    pub fn get_hand_type(&self) -> HandType {
        let hand_type = self.get_hand_type_legacy();
        if self.jacks_as_joker {
            self.improve_jacks_hands(hand_type)
        } else {
            hand_type
        }
    }

    fn improve_jacks_hands(&self, hand_type: HandType) -> HandType {
        let num_of_jokers = self
            .hand
            .iter()
            .filter(|card| (*card).eq(&Card::Jack))
            .count();

        if num_of_jokers == 0 {
            hand_type
        } else {
            match hand_type {
                HandType::Five => HandType::Five, // There are 5 jacks, it's a Five
                HandType::HighCard => HandType::OnePair, // As long as 1 joker exists, this will become a 1pair
                HandType::Four => HandType::Five,        // A Five is always bigger than Four
                HandType::FullHouse => HandType::Five, // As long as Joker is part of a Fullhouse, it's a Five
                HandType::Three => HandType::Four, // As long as Joker is part of a Three Hand, it's a Four
                HandType::TwoPair => {
                    if num_of_jokers == 1 {
                        HandType::FullHouse
                    } else {
                        HandType::Four
                    }
                }
                HandType::OnePair => HandType::Three, // A pair of joker -> Three, 1 joker -> Three
            }
        }
    }

    fn get_hand_type_legacy(&self) -> HandType {
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

            jacks_as_joker: false,
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
                if self.jacks_as_joker {
                    if card1 == &Card::Jack {
                        return Ordering::Less;
                    } else if card2 == &Card::Jack {
                        return Ordering::Greater;
                    }
                }
                    return card1.cmp(card2);
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.jacks_as_joker != other.jacks_as_joker {
            None
        } else {
            Some(self.cmp(other))
        }
    }
}

type BidType = u32;

fn parse_hand_bid(hand_bid_str: &str, use_joker: bool) -> (Hand, BidType) {
    if let Some((hand_data, bid_str)) = hand_bid_str.split_once(' ') {
        let mut hand = Hand::from(hand_data);
        if use_joker {
            hand.enable_joker();
        }
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
    let hands_to_bids = hands_bid_data
        .split("\n")
        .map(|line| parse_hand_bid(line, false))
        .fold(
            BTreeMap::new(),
            move |mut ranked_handbids_map, (hand, bid)| {
                ranked_handbids_map.insert(hand, bid);
                ranked_handbids_map
            },
        );

    let bid_returns = calculate_bid_returns(hands_to_bids.values().collect());
    return bid_returns.iter().sum();
}

pub fn compute_hands_bid_jokers(hands_bid_data: &str) -> u32 {
    let hands_to_bids = hands_bid_data
        .split("\n")
        .map(|line| parse_hand_bid(line, true))
        .fold(
            BTreeMap::new(),
            move |mut ranked_handbids_map, (hand, bid)| {
                ranked_handbids_map.insert(hand, bid);
                ranked_handbids_map
            },
        );
    
    let bid_returns = calculate_bid_returns(hands_to_bids.values().collect());
    return bid_returns.iter().sum();
}

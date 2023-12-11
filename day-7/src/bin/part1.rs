use std::cmp::Ordering;
use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse, 
    FourOfAKind,
    FiveOfAKind
}

#[derive(PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    cards_map: HashMap<char, u8>,
    bid: u32,
    hand_type: HandType
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type < other.hand_type {
            Ordering::Less
        } else {
            let strengths = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
            let mut self_strengths = Vec::new();
            for c in self.cards.chars() {
                self_strengths.push(strengths.iter().position(|&ch| ch == c).unwrap());
            }

            let mut other_strengths = Vec::new();
            for c in other.cards.chars() {
                other_strengths.push(strengths.iter().position(|&ch| ch == c).unwrap());
            }

            self_strengths.iter().cmp(other_strengths.iter())
        }
    }
}

fn main() {
    let mut hands: Vec<Hand> = vec![];
    let content = read_to_string("../data.txt").unwrap();
    for line in content.lines() {
        let (cards, bid) = line.split_once(" ").unwrap();
        let mut hand = Hand{
            cards: cards,
            cards_map: HashMap::new(),
            bid: bid.parse().unwrap(),
            hand_type: HandType::HighCard
        };
        
        for card in cards.chars() {
            hand.cards_map.entry(card).and_modify(|x| *x += 1).or_insert(1);
        }
        
        let counts = hand.cards_map.values().collect::<Vec<_>>();
        let max: u8 = **counts.iter().max().unwrap();
        hand.hand_type = match counts.len() {
            1 => HandType::FiveOfAKind,
            2 if max == 4 => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if max == 3 => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Unknown type")
        };

        hands.push(hand);
    }

    hands.sort();

    let mut sum = 0;
    for i in 1..=hands.len() {
        sum += (hands[i-1].bid as usize) * i;
        println!("hand: {}, bid: {}", hands[i-1].cards, hands[i-1].bid);
    }

    println!("Total winning: {}", sum);
}

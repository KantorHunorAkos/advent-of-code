use std::fs::read_to_string;
use std::collections::HashMap;

enum HandType {
    HighCard = 1,
    OnePair = 1 << 1,
    TwoPair = 1 << 2,
    ThreeOfAKind = 1 << 3,
    FourOfAKind = 1 << 4,
    FiveOfAKind = 1 << 5,
    FullHouse = (1 << 3) & (1 << 2) 
}

struct Hand<'a> {
    cards: &'a str,
    cards_map: HashMap<char, u8>,
    bid: u32,
    hand_type: HandType
}

fn main() {
    let mut hands: Vec<Hand> = vec![];
    let content = read_to_string("../test_data.txt").unwrap();
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
        
        let counts = hand.cards_map.into_values().collect::<Vec<_>>();
        let max = counts.iter().max().unwrap();
        hand.hand_type = match cards.len() {
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
    for hand in hands {
        for card in hand.cards_map.values() {
            println!("{}", card);
        }
        println!();
    }
}

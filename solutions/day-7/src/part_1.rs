use std::collections::HashMap;

pub fn solution(input: &str) -> u64 {
    let mut hands: Vec<(Hand, u32)> = parse_input(input);
    hands.sort();

    let mut result: u64 = 0;

    for (i, (_, bid)) in hands.into_iter().enumerate() {
        result += bid as u64 * (i as u64 + 1)
    }
    return result;
}

fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    let mut result: Vec<(Hand, u32)> = Vec::new();
    for line in input.lines() {
        let (hand_str, bid_str) = line.split_once(" ").unwrap();
        result.push((Hand::from(hand_str), bid_str.parse::<u32>().unwrap()));
    }
    return result;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        return match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid char for from conversion into Card"),
        };
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    kind: HandType,
    cards: [Card; 5],
}

impl From<&str> for Hand {
    fn from(card_str: &str) -> Self {
        let mut cards: [Card; 5] = [Card::Two; 5];
        let mut map: HashMap<Card, u8> = HashMap::new();

        for (i, c) in card_str.chars().enumerate() {
            cards[i] = Card::from(c);
            let card_count = map.get(&cards[i]).unwrap_or(&0u8);
            map.insert(cards[i], card_count + 1);
        }
        return Hand {
            cards,
            kind: match map.len() {
                1 => HandType::FiveOfAKind,
                2 => {
                    let mut counts: Vec<u8> = map.into_values().collect();
                    counts.sort();
                    match counts[..] {
                        [1, 4] => HandType::FourOfAKind,
                        [2, 3] => HandType::FullHouse,
                        _ => todo!(),
                    }
                }
                3 => {
                    let mut counts: Vec<u8> = map.into_values().collect();
                    counts.sort();
                    match counts[..] {
                        [1, 1, 3] => HandType::ThreeOfAKind,
                        [1, 2, 2] => HandType::TwoPair,
                        _ => todo!(),
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => panic!("Unreachable set length"),
            },
        };
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

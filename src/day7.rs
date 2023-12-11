use std::collections::HashMap;

#[allow(dead_code)]
fn get_card_strength(card: u8) -> u8 {
    if card >= b'0' && card <= b'9' {
        card - b'0'
    } else {
        match card {
            b'A' => 13,
            b'K' => 12,
            b'Q' => 11,
            b'J' => 1,
            b'T' => 10,
            _ => unreachable!("never happens!")
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard
}

#[allow(dead_code)]
fn get_hand_type(hand: &str) -> (HandType, [u8; 5]) {
    let mut map = HashMap::<u8, u8>::new();
    let mut bytes = [0u8; 5];
    for (i, byte) in hand.as_bytes().iter().enumerate() {
        bytes[i] = *byte;
        if let Some(x) = map.get_mut(byte) {
            *x += 1;
        } else {
            map.insert(*byte, 1);
        }
    }
    let mut vec = map.clone().into_iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    let kind = match vec[0].1 {
        5 => HandType::FiveOfAKind,
        4 => {
            if let Some(_) = map.get(&b'J') {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            }
        }
        3 => {
            if let Some(count) = map.get(&b'J') {
                if *count == 3 {
                    let count = vec[1].1;
                    if count == 2 {
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                } else if *count == 2 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            } else {
                match vec[1].1 {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind
                }
            }
        }
        2 => {
            if let Some(count) = map.get(&b'J') {
                if *count == 2 {
                    let count = vec[1].1;
                    if count == 2 {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else {
                    let count = vec[1].1;
                    if count == 1 {
                        HandType::ThreeOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
            } else {
                match vec[1].1 {
                    2 => HandType::TwoPairs,
                    _ => HandType::OnePair
                }
            }
        }
        1 => {
            if let Some(_) = map.get(&b'J') {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
        _ => unreachable!("never happens!")
    };
    (kind, bytes)
}

#[allow(dead_code)]
fn compare_cards(a: &[u8; 5], b: &[u8; 5]) -> std::cmp::Ordering {
    for (idx, card) in a.iter().enumerate() {
        match get_card_strength(*card).cmp(&get_card_strength(b[idx])) {
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
        }
    }
    return std::cmp::Ordering::Equal;
}

#[allow(dead_code)]
fn compare_hands(a: &(HandType, [u8; 5]), b: &(HandType, [u8; 5])) -> std::cmp::Ordering {
    match a.0 {
        HandType::FiveOfAKind => {
            match b.0 {
                HandType::FiveOfAKind => compare_cards(&a.1, &b.1),
                _ => std::cmp::Ordering::Greater
            }
        }
        HandType::FourOfAKind => {
            match b.0 {
                HandType::FiveOfAKind => std::cmp::Ordering::Less,
                HandType::FourOfAKind => compare_cards(&a.1, &b.1),
                _ => std::cmp::Ordering::Greater
            }
        }
        HandType::FullHouse => {
            match b.0 {
                HandType::FiveOfAKind => std::cmp::Ordering::Less,
                HandType::FourOfAKind => std::cmp::Ordering::Less,
                HandType::FullHouse => compare_cards(&a.1, &b.1),
                _ => std::cmp::Ordering::Greater
            }
        }
        HandType::ThreeOfAKind => {
            match b.0 {
                HandType::FiveOfAKind => std::cmp::Ordering::Less,
                HandType::FourOfAKind => std::cmp::Ordering::Less,
                HandType::FullHouse => std::cmp::Ordering::Less,
                HandType::ThreeOfAKind => compare_cards(&a.1, &b.1),
                _ => std::cmp::Ordering::Greater
            }
        }
        HandType::TwoPairs => {
            match b.0 {
                HandType::FiveOfAKind => std::cmp::Ordering::Less,
                HandType::FourOfAKind => std::cmp::Ordering::Less,
                HandType::FullHouse => std::cmp::Ordering::Less,
                HandType::ThreeOfAKind => std::cmp::Ordering::Less,
                HandType::TwoPairs => compare_cards(&a.1, &b.1),
                _ => std::cmp::Ordering::Greater
            }
        }
        HandType::OnePair => {
            match b.0 {
                HandType::FiveOfAKind => std::cmp::Ordering::Less,
                HandType::FourOfAKind => std::cmp::Ordering::Less,
                HandType::FullHouse => std::cmp::Ordering::Less,
                HandType::ThreeOfAKind => std::cmp::Ordering::Less,
                HandType::TwoPairs => std::cmp::Ordering::Less,
                HandType::OnePair => compare_cards(&a.1, &b.1),
                _ => std::cmp::Ordering::Greater
            }
        }
        HandType::HighCard => {
            match b.0 {
                HandType::HighCard => compare_cards(&a.1, &b.1),
                _ => std::cmp::Ordering::Less,
            }
        }
    }
}

#[allow(dead_code)]
pub fn soln(lines: Vec<String>) -> (u64, u64) {
    let mut hands = Vec::<((HandType, [u8; 5]), u64)>::new();
    for line in lines {
        let (hand, bet) = line.trim().split_once(" ").unwrap();
        let hand = get_hand_type(hand);
        hands.push((hand, bet.trim().parse::<u64>().unwrap()));
    }
    hands.sort_by(|a, b| compare_hands(&a.0, &b.0));
    let mut total = 0;
    for (idx, ((_, cards), rank)) in hands.iter().enumerate() {
        //println!("{}{}{}{}{}", cards[0] as char, cards[1] as char, cards[2] as char, cards[3] as char, cards[4] as char);
        total += (idx as u64 + 1) * (*rank);
    }
    (total, 0)
}

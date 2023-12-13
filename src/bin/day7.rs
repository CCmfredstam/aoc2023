use std::{fs::read_to_string, collections::HashMap};

#[derive(Eq, PartialEq, PartialOrd, Ord)]
#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, PartialEq, PartialOrd)]
#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i64,
    hand_type: HandType,
    joker_active: bool,
    highest_card: char,
}


fn determine_hand_type(card_counts: HashMap<char, usize>) -> HandType {
    if card_counts.values().any(|&count| count == 5) {
        HandType::FiveOfAKind
    } else if card_counts.values().any(|&count| count == 4) {
        HandType::FourOfAKind
    } else if card_counts.values().any(|&count| count == 3)
        && card_counts.values().any(|&count| count == 2)
    {
        HandType::FullHouse
    } else if card_counts.values().any(|&count| count == 3) {
        HandType::ThreeOfAKind
    } else if card_counts.values().filter(|&&count| count == 2).count() == 2 {
        HandType::TwoPair
    } else if card_counts.values().any(|&count| count == 2) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn count_cards(hand: &str) -> HashMap<char, usize> {
    let mut card_counts = HashMap::new();

    let number_of_jokers = hand.chars().filter(|&c| c == 'J').count();
    println!("Found {} of J in hand {:?}", number_of_jokers, hand);
    for card in hand.chars() {
        *card_counts.entry(card).or_insert(0) += 1;
    }

    for (card, count) in &mut card_counts {
        if card.to_string() != 'J'.to_string() {
            *count += number_of_jokers;
        }
    }

    card_counts
}

impl Hand {
    fn new(hand: Vec<String>) -> Self {
        let card_count = count_cards(&hand[0]);
        let type_of_hand = determine_hand_type(card_count.clone());
        Self {
            cards: hand[0].to_owned(),
            bid: hand[1].parse().unwrap(),
            hand_type: type_of_hand,
            joker_active: hand[0].contains('J'),
            highest_card: {
                let mut highest_card = 'x';
                let mut highest_card_num = 0;
                for card in card_count {
                    if card.1 > highest_card_num {
                        highest_card = card.0;
                        highest_card_num = card.1;
                    }
                }
                highest_card
            },
        }
    }
}

fn card_value(card_face: char) -> i64 {
    match card_face {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        _ => (card_face as u8 - b'0').into()
    }
}

fn sort_hands(hand_a: &Hand, hand_b: &Hand) -> std::cmp::Ordering{
    if hand_a.hand_type == hand_b.hand_type {
        let x: std::cmp::Ordering = std::cmp::Ordering::Equal;
        for (a_ch, b_ch) in hand_a.cards.chars().zip(hand_b.cards.chars()) {
            if a_ch != b_ch {
                let a_numeric = match a_ch {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if hand_a.joker_active {
                            card_value(hand_a.highest_card)
                        } else {
                            1
                        }
                    },
                    'T' => 10,
                    _ => (a_ch as u8 - b'0').into()
                };
                let b_numeric = match b_ch {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if hand_a.joker_active {
                            card_value(hand_a.highest_card)
                        } else {
                            1
                        }
                    },
                    'T' => 10,
                    _ => (b_ch as u8 - b'0').into()
                };
                return a_numeric.cmp(&b_numeric).reverse();
            }
        }
        x
    } else {
        hand_a.hand_type.cmp(&hand_b.hand_type)
    }
}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day7.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let _lines: Vec<String> = vec!["32T3K 765".to_string(),
                                  "T55J5 684".to_string(),
                                  "KK677 28".to_string(),
                                  "KTJJT 220".to_string(),
                                  "QQQJA 483".to_string()];

    // Fetch all hands in to vector
    let mut all_hands: Vec<Hand> = vec![];
    for line in lines {
        let h: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        all_hands.push(Hand::new(h));
    }

    println!("Before sort");
    for hand in &all_hands {
        println!("{:?}", hand);
    }

    all_hands.sort_by(sort_hands);

    println!("After sort");
    for hand in &all_hands {
        println!("{:?}", hand);
    }

    // Calculate totalt winning
    //  Sum(Bid * rank)   ->  (lowest rank = 1)
    let mut winnings: i64 = 0;
    for (rank, hand) in all_hands.iter().rev().enumerate() {
        println!("Rank {:?} -> Hand {:?} -> Hand bid {:?} -> Winning {:?}", rank+1, hand.cards, hand.bid, (rank+1) as i64 * hand.bid);
        winnings += (rank+1) as i64 * hand.bid;
    }


    println!("Part1: {}", winnings);
    println!("Part1 test expected: 6440");

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day7.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let _lines: Vec<String> = vec!["32T3K 765".to_string(),
                                  "T55J5 684".to_string(),
                                  "KK677 28".to_string(),
                                  "KTJJT 220".to_string(),
                                  "QQQJA 483".to_string()];

    // Fetch all hands in to vector
    let mut all_hands: Vec<Hand> = vec![];
    for line in lines {
        let h: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        all_hands.push(Hand::new(h));
    }

    println!("Before sort");
    for hand in &all_hands {
        println!("{:?}", hand);
    }

    all_hands.sort_by(sort_hands);

    println!("After sort");
    for hand in &all_hands {
        println!("{:?}", hand);
    }

    // Calculate totalt winning
    let mut winnings: i64 = 0;
    for (rank, hand) in all_hands.iter().rev().enumerate() {
        println!("Rank {:?} -> Hand {:?} -> Hand bid {:?} -> Winning {:?}", rank+1, hand.cards, hand.bid, (rank+1) as i64 * hand.bid);
        winnings += (rank+1) as i64 * hand.bid;
    }


    let too_big = 246199075;
    // "Not right": 245789439
    println!("Part2: {}", winnings);
    println!("Too big vs. {} = {}", too_big, winnings>=too_big);
    println!("Part2 test expected: 5905");

}

fn main() {
    main_part1();
    main_part2();
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}
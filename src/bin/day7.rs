use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven= 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

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
}


fn determine_hand_type(hand: &str) -> HandType {
    let card_counts = count_cards(hand);

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

    for card in hand.chars() {
        *card_counts.entry(card).or_insert(0) += 1;
    }

    card_counts
}

impl Hand {
    fn new(hand: Vec<String>) -> Self {
        let type_of_hand = determine_hand_type(&hand[0]);
        Self { cards: hand[0].to_owned(), bid: hand[1].parse().unwrap(), hand_type: type_of_hand }
    }

    fn numeric_value(&self) -> i64 {
        let mut value: i64 = 0;
        for card in self.cards.chars() {
            value += match card {
                'A' => Card::Ace as i64,
                'K' => Card::King as i64,
                'Q' => Card::Queen as i64,
                'J' => Card::Jack as i64,
                'T' => Card::Ten as i64,
                '9' => Card::Nine as i64,
                '8' => Card::Eight as i64,
                '7' => Card::Seven as i64,
                '6' => Card::Six as i64,
                '5' => Card::Five as i64,
                '4' => Card::Four as i64,
                '3' => Card::Three as i64,
                '2' => Card::Two as i64,
                _ => 0,
            }
        }
        value
    }
}

fn sort_hands(hand_a: &Hand, hand_b: &Hand) -> std::cmp::Ordering{
    if hand_a.hand_type == hand_b.hand_type {
        let x: std::cmp::Ordering = std::cmp::Ordering::Less;
        for (a_ch, b_ch) in hand_a.cards.chars().zip(hand_b.cards.chars()) {
            if a_ch != b_ch {
                let a_numeric = match a_ch {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => a_ch as u8 - '0' as u8
                };
                let b_numeric = match b_ch {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => b_ch as u8 - '0' as u8
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
        let h: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        all_hands.push(Hand::new(h));
    }

    println!("Before sort");
    for hand in &all_hands {
        println!("{:?}", hand);
    }

    all_hands.sort_by(|a, b| sort_hands(a,b));

    println!("After sort");
    for hand in &all_hands {
        println!("{:?}", hand);
    }

    // Calculate totalt wining
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


    println!("Part2: {}", 0);

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
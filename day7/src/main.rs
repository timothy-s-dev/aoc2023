use std::cmp::Ordering;

#[derive(Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Card {
    fn value(&self) -> i64 {
        match self {
            Card::Joker => -1,
            Card::Two => 0,
            Card::Three => 1,
            Card::Four => 2,
            Card::Five => 3,
            Card::Six => 4,
            Card::Seven => 5,
            Card::Eight => 6,
            Card::Nine => 7,
            Card::Ten => 8,
            Card::Jack => 9,
            Card::Queen => 10,
            Card::King => 11,
            Card::Ace => 12,
        }
    }

    fn parse(c: char, is_part_one: bool) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => if is_part_one { Card::Jack } else { Card::Joker },
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card symbol: {}", c),
        }
    }
}

#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl HandType {
    fn rank(&self) -> i64 {
        match self {
            HandType::HighCard => 0,
            HandType::OnePair => 1,
            HandType::TwoPairs => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        }
    }

    fn identify(cards: &[Card; 5]) -> HandType {
        let mut jokers = 0;
        let mut counts = [0; 13];
        for card in cards.iter() {
            match card {
                Card::Joker => jokers += 1,
                _ => counts[card.value() as usize] += 1,
            }
        }
        let pairs = counts.iter().filter(|count| **count == 2).count();

        if counts.iter().any(|count| *count == 5 - jokers) {
            return HandType::FiveOfAKind;
        } else if counts.iter().any(|count| *count == 4 - jokers) {
            return HandType::FourOfAKind;
        } else if counts.iter().filter(|count| **count > 0).count() == 2 {
            return HandType::FullHouse;
        } else if counts.iter().any(|count| *count == 3 - jokers) {
            return HandType::ThreeOfAKind;
        } else if pairs == 2 - jokers {
            return HandType::TwoPairs;
        } else if counts.iter().any(|count| *count == 2 - jokers) {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: i64,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type.rank() == other.hand_type.rank() &&
            self.cards[0].value() == other.cards[0].value() &&
            self.cards[1].value() == other.cards[1].value() &&
            self.cards[2].value() == other.cards[2].value() &&
            self.cards[3].value() == other.cards[3].value() &&
            self.cards[4].value() == other.cards[4].value()
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type.rank() < other.hand_type.rank() {
            return Some(Ordering::Less);
        } else if self.hand_type.rank() > other.hand_type.rank() {
            return Some(Ordering::Greater);
        } else {
            for i in 0..5 {
                if self.cards[i].value() < other.cards[i].value() {
                    return Some(Ordering::Less);
                } else if self.cards[i].value() > other.cards[i].value() {
                    return Some(Ordering::Greater);
                }
            }
        }
        Some(Ordering::Equal)
    }
}

impl Hand {
    fn parse(line: &str, is_part_one: bool) -> Hand {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let cards = [
            Card::parse(parts[0].chars().nth(0).unwrap(), is_part_one),
            Card::parse(parts[0].chars().nth(1).unwrap(), is_part_one),
            Card::parse(parts[0].chars().nth(2).unwrap(), is_part_one),
            Card::parse(parts[0].chars().nth(3).unwrap(), is_part_one),
            Card::parse(parts[0].chars().nth(4).unwrap(), is_part_one),
        ];
        let hand_type = HandType::identify(&cards);
        let bid = parts[1].parse::<i64>().unwrap();
        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = common::read_file_line_by_line(&input_file_path);
    let mut hands = lines.iter().map(|line| Hand::parse(line, is_part_one)).collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let total_winnings = hands.iter().enumerate().map(|(i, hand)| {
        (i + 1) as i64 * hand.bid
    }).sum::<i64>();
    println!("Total winnings: {}", total_winnings);
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning_numbers_you_have: Vec<u32>,
    score: u32
}

impl Card {
    fn parse(line: &str) -> Card {
        let mut parts = line.split(": ");
        let id = parts.next().unwrap()[4..].replace(" ", "").parse::<u32>().unwrap();
        let mut sub_parts = parts.next().unwrap().split(" | ");
        let winning_numbers = sub_parts.next().unwrap().split(" ")
            .filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();
        let numbers_you_have = sub_parts.next().unwrap().split(" ")
            .filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();
        let winning_numbers_you_have: Vec<u32> = winning_numbers.iter()
            .filter(|n| numbers_you_have.contains(n)).map(|n| *n).collect();
        let score = if winning_numbers_you_have.len() > 0 {
            2_u32.pow(winning_numbers_you_have.len() as u32 - 1)
        } else {
            0
        };
        Card {
            id,
            winning_numbers_you_have,
            score
        }
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();

    let cards = common::read_file_line_by_line(&input_file_path).iter()
        .map(|l| Card::parse(l)).collect::<Vec<Card>>();

    if is_part_one {
        println!("Total Score: {}", cards.iter().map(|c| c.score).sum::<u32>());
    } else {
        let mut card_scores: Vec<u32> = vec![0; cards.len()];
        for i in (0..cards.len()).rev() {
            let card = &cards[i];
            let mut score = card.winning_numbers_you_have.len() as u32;
            for j in 0..score {
                score += card_scores[1 + i + j as usize];
            }
            card_scores[i] = score;
        }
        println!("Total Scratch Cards: {}", card_scores.iter().sum::<u32>() + cards.len() as u32);
    }
}

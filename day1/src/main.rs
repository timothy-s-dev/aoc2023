struct DigitMapping {
    string: &'static str,
    value: u32,
}

const DIGIT_MAPPINGS: [DigitMapping;18] = [
    DigitMapping { string: "one", value: 1 },
    DigitMapping { string: "two", value: 2 },
    DigitMapping { string: "three", value: 3 },
    DigitMapping { string: "four", value: 4 },
    DigitMapping { string: "five", value: 5 },
    DigitMapping { string: "six", value: 6 },
    DigitMapping { string: "seven", value: 7 },
    DigitMapping { string: "eight", value: 8 },
    DigitMapping { string: "nine", value: 9 },
    DigitMapping { string: "1", value: 1 },
    DigitMapping { string: "2", value: 2 },
    DigitMapping { string: "3", value: 3 },
    DigitMapping { string: "4", value: 4 },
    DigitMapping { string: "5", value: 5 },
    DigitMapping { string: "6", value: 6 },
    DigitMapping { string: "7", value: 7 },
    DigitMapping { string: "8", value: 8 },
    DigitMapping { string: "9", value: 9 },
];

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines: Vec<String> = common::read_file_line_by_line(&input_file_path);
    let values = lines.iter().map(|l| {
        let first: u32 =
            if is_part_one { first_digit_in_line(l) }
            else { find_first_digit_mapping(l).unwrap() };
        let last: u32 =
            if is_part_one { last_digit_in_line(l) }
            else { find_last_digit_mapping(l).unwrap() };
        first * 10 + last
    });
    let mut sum: u32 = 0;
    for v in values {
        sum += v;
    }
    println!("sum: {}", sum);
}

fn first_digit_in_line(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            first_digit = c.to_digit(10).unwrap();
            break;
        }
    }
    first_digit
}
fn last_digit_in_line(line: &str) -> u32 {
    let mut last_digit: u32 = 0;
    for c in line.chars().rev() {
        if c.is_digit(10) {
            last_digit = c.to_digit(10).unwrap();
            break;
        }
    }
    last_digit
}

fn find_first_digit_mapping(line: &str) -> Option<u32> {
    let chars: Vec<char> = line.chars().collect();
    for (i, _) in chars.iter().enumerate() {
        for mapping in DIGIT_MAPPINGS.iter() {
            if line[i..].starts_with(mapping.string) {
                return Some(mapping.value);
            }
        }
    }
    None
}
fn find_last_digit_mapping(line: &str) -> Option<u32> {
    let chars: Vec<char> = line.chars().collect();
    for (i, _) in chars.iter().enumerate().rev() {
        for mapping in DIGIT_MAPPINGS.iter() {
            if line[..=i].ends_with(mapping.string) {
                return Some(mapping.value);
            }
        }
    }
    None
}
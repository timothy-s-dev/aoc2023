type History = Vec<i64>;

fn parse_history(line: &str) -> History {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn get_history_derivative(history: &History) -> History {
    let mut derivative = Vec::new();
    for i in 1..history.len() {
        derivative.push(history[i] - history[i - 1]);
    }
    derivative
}

fn is_all_zeros(history: &History) -> bool {
    for i in history {
        if *i != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = common::read_file_line_by_line(&input_file_path);
    let histories = lines.iter().map(|line| parse_history(line)).collect::<Vec<History>>();

    let mut result: i64 = 0;
    for history in histories {
        let mut derivatives: Vec<History> = Vec::new();
        derivatives.push(get_history_derivative(&history));
        while !is_all_zeros(derivatives.last().unwrap()) {
            derivatives.push(get_history_derivative(derivatives.last().unwrap()));
        }

        derivatives.reverse();
        let mut last_value: i64 = 0;
        for derivative in derivatives {
            if is_part_one {
                last_value = last_value + derivative.last().unwrap();
            } else {
                last_value = derivative.first().unwrap() - last_value;
            }
        }
        last_value = history.first().unwrap() - last_value;

        result += last_value;
    }

    println!("Result: {}", result);
}

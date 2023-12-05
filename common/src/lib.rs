use std::io::BufRead;

pub fn get_input_file_path() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Missing part number argument");
    }
    let part_number = args[1].parse::<u32>().unwrap();
    let is_test = args.len() > 2 && args[2] == "test";
    if is_test {
        format!("example{}.txt", part_number)
    } else {
        "input.txt".to_string()
    }
}

pub fn is_part_one() -> bool {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Missing part number argument");
    }
    let part_number = args[1].parse::<u32>().unwrap();
    part_number == 1
}

pub fn read_file_line_by_line(path: &str) -> Vec<String> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

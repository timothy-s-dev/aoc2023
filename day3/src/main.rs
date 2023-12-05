#[derive(Debug)]
struct PartNumber {
    value: u32,
    line: usize,
    column: usize
}

impl PartNumber {
    fn width(&self) -> usize {
        self.value.to_string().len()
    }

    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        if self.line < symbol.line - 1 || self.line > symbol.line + 1 {
            false
        } else if self.column + self.width() - 1 < symbol.column - 1 || self.column > symbol.column + 1 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    line: usize,
    column: usize
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut line_number: usize = 0;
    let mut number: u32 = 0;
    let mut number_column: usize = 0;

    for line in common::read_file_line_by_line(&input_file_path) {
        for (column, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                if number == 0 { number_column = column; }
                number *= 10;
                number += character.to_digit(10).unwrap();
            } else if number > 0 {
                part_numbers.push(PartNumber {
                    value: number,
                    line: line_number,
                    column: number_column
                });
                number = 0;
            }
            if !character.is_digit(10) && character != '.' {
                symbols.push(Symbol {
                    value: character,
                    line: line_number,
                    column
                });
            }
        }
        if number > 0 {
            part_numbers.push(PartNumber {
                value: number,
                line: line_number,
                column: number_column
            });
            number = 0;
        }
        line_number += 1;
    }

    if is_part_one {
        let mut part_number_sum: u32 = 0;
        for part_number in &part_numbers {
            if symbols.iter().any(|x| part_number.is_adjacent(x)) {
                part_number_sum += part_number.value;
            }
        }
        println!("Part number sum: {}", part_number_sum);
    } else {
        let mut gear_ratios_sum: u32 = 0;
        for symbol in &symbols {
            if symbol.value != '*' { continue; }
            let adjacent_numbers: Vec<u32> = part_numbers.iter()
                .filter(|x| x.is_adjacent(&symbol))
                .map(|x| x.value)
                .collect();
            if adjacent_numbers.len() == 2 {
                gear_ratios_sum += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
        println!("Gear ratio sum: {}", gear_ratios_sum);
    }
}

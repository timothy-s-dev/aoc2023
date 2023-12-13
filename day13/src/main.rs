fn find_reflection_index(values: &Vec<u64>, with_smudge: bool) -> Option<u64> {
    for center in 1..values.len() {
        let mut found_center = true;
        let mut has_smudge = false;
        for right in center..values.len() {
            let offset = right - center + 1;
            if offset > center {
                if !with_smudge || has_smudge {
                    return Some(center as u64)
                } else {
                    found_center = false;
                    break;
                };
            }
            let left = center - offset;
            if values[left] != values[right] {
                // if the bit-xor is a power of two, then there is exactly one tile/bit different
                if with_smudge && !has_smudge && (values[left] ^ values[right]).is_power_of_two()  {
                    has_smudge = true;
                } else {
                    found_center = false;
                    break;
                }
            }
        }
        if found_center && (!with_smudge || has_smudge) {
            return Some(center as u64);
        }
    }
    None
}

struct Field {
    cells: Vec<Vec<char>>,
    row_values: Vec<u64>,
    col_values: Vec<u64>,
}

impl Field {
    fn parse(lines: &Vec<String>) -> Field {
        let mut cells = Vec::new();
        let mut row_values = Vec::new();
        let mut col_values = Vec::new();

        for line in lines {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            cells.push(row);
        }

        for i in 0..cells.len() {
            let mut row_value = 0;
            for j in 0..cells[i].len() {
                if cells[i][j] == '#' {
                    row_value += 1 << j;
                }
            }
            row_values.push(row_value);
        }

        for j in 0..cells[0].len() {
            let mut col_value = 0;
            for i in 0..cells.len() {
                if cells[i][j] == '#' {
                    col_value += 1 << i;
                }
            }
            col_values.push(col_value);
        }

        Field {
            cells,
            row_values,
            col_values,
        }
    }

    fn find_horizontal_reflection_index(&self, with_smudge: bool) -> Option<u64> {
        find_reflection_index(&self.col_values, with_smudge)
    }

    fn find_vertical_reflection_index(&self, with_smudge: bool) -> Option<u64> {
        find_reflection_index(&self.row_values, with_smudge)
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = &common::read_file_line_by_line(&input_file_path);

    let fields = lines.split(|line| line.is_empty())
        .map(|lines| Field::parse(&lines.to_vec()));

    for (i, field) in fields.clone().enumerate() {
        if let Some(index) = field.find_horizontal_reflection_index(!is_part_one) {
            println!("Field {} horizontal reflection index: {}", i, index);
        } else if let Some(index) = field.find_vertical_reflection_index(!is_part_one) {
            println!("Field {} vertical reflection index: {}", i, index);
        } else {
            println!("WARNING!!!  Field {} has no reflection index", i);
        }
    }

    let horizontal_value: u64 = fields.clone()
        .filter_map(|field| field.find_horizontal_reflection_index(!is_part_one))
        .sum();
    let vertical_value: u64 = fields.clone()
        .filter_map(|field| field.find_vertical_reflection_index(!is_part_one))
        .sum();

    println!("Horizontal value: {}", horizontal_value);
    println!("Vertical value: {}", vertical_value);

    println!("Solution: {}", vertical_value * 100 + horizontal_value);
}

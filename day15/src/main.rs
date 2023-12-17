fn hash(input: &str) -> u32 {
    let mut hash = 0;
    for c in input.chars() {
        hash += c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[derive(Debug, Clone)]
enum Instruction {
    Set(String, u32),
    Clear(String),
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        if input.contains("-") {
            Self::Clear(input.to_string().replace("-", ""))
        } else {
            let parts = input.split("=").collect::<Vec<&str>>();
            Self::Set(parts[0].to_string(), parts[1].parse::<u32>().unwrap())
        }
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let input = &common::read_file_line_by_line(&input_file_path)[0];

    let instructions = input.split(",").collect::<Vec<&str>>();

    if is_part_one {
        let result = instructions.iter().map(|s| hash(s)).sum::<u32>();
        println!("Result: {}", result);
    } else {
        let instructions = instructions.iter().map(|s| Instruction::parse(s)).collect::<Vec<Instruction>>();
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
        for instruction in instructions {
            //println!("{:?}", instruction);
            match instruction {
                Instruction::Set(label, focal_length) => {
                    let hash = hash(&label) as usize;
                    let lens_index = boxes[hash].iter().position(|lens| lens.label == label);
                    if let Some(index) = lens_index {
                        boxes[hash][index].focal_length = focal_length;
                    } else {
                        let lens = Lens { label, focal_length };
                        let new_lens_index = boxes[hash].len();
                        boxes[hash].insert(new_lens_index, lens);
                    }
                },
                Instruction::Clear(label) => {
                    let hash = hash(&label) as usize;
                    let lens_index = boxes[hash].iter().position(|lens| lens.label == label);
                    if let Some(index) = lens_index {
                        boxes[hash].remove(index);
                    }
                },
            }

            /* Print all boxes each step
            for (i, b) in boxes.iter().enumerate() {
                if b.len() > 0 {
                    println!("  Box {}: {:?}", i, b);
                }
            }*/
        }

        let mut focusing_power = 0;
        for (box_index, b) in boxes.iter().enumerate() {
            for (slot_index, lens) in b.iter().enumerate() {
                let box_number = (box_index + 1) as u32;
                let slot_number = (slot_index + 1) as u32;
                focusing_power += box_number * slot_number * lens.focal_length;
            }
        }
        println!("Result: {}", focusing_power);
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = &common::read_file_line_by_line(&input_file_path);

    println!("TODO: Complete puzzle (part {}) using file: {}",
             if is_part_one { 1 } else { 2 },
             input_file_path
    );
}

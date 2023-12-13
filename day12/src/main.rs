use memoize::memoize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn parse(c: char) -> Self {
        match c {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Invalid condition: {}", c),
        }
    }
}

fn debug_log(depth: usize, debug: bool, message: &str) {
    if debug {
        println!("{}{}", " ".repeat(depth), message);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DataSet {
    spring_conditions: Vec<Condition>,
    damaged_sets: Vec<u32>,
}

impl DataSet {
    fn parse(line: &str, unfold: bool) -> Self {
        let mut parts = line.split(" ");
        let times = if unfold { 5 } else { 1 };
        let spring_conditions = std::iter::repeat(parts.next().unwrap())
            .take(times).collect::<Vec<&str>>()
            .join("?").chars()
            .map(|c| Condition::parse(c)).collect::<Vec<Condition>>();
        let damaged_sets = std::iter::repeat(parts.next().unwrap())
            .take(times).collect::<Vec<&str>>()
            .join(",").split(",")
            .map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        Self {
            spring_conditions,
            damaged_sets,
        }
    }

    fn trim_operational(&self) -> Self {
        let mut spring_conditions = self.spring_conditions.clone();
        while spring_conditions.len() > 0 && *spring_conditions.first().unwrap() == Condition::Operational {
            spring_conditions.remove(0);
        }
        while spring_conditions.len() > 0 && *spring_conditions.last().unwrap() == Condition::Operational {
            spring_conditions.pop();
        }
        Self {
            spring_conditions,
            damaged_sets: self.damaged_sets.clone(),
        }
    }

    fn can_fit_next_damaged_set(&self) -> bool {
        self.spring_conditions.len() >= self.damaged_sets[0] as usize
            && self.spring_conditions[0..self.damaged_sets[0] as usize].iter().all(|c| *c != Condition::Operational)
            && (self.spring_conditions.len() == self.damaged_sets[0] as usize
                || self.spring_conditions[self.damaged_sets[0] as usize] != Condition::Damaged)
    }

    fn has_damaged_set_but_no_springs(&self) -> bool {
        self.spring_conditions.len() == 0 && self.damaged_sets.len() > 0
    }

    fn has_damaged_springs(&self) -> bool {
        self.spring_conditions.iter().any(|s| *s == Condition::Damaged)
    }
}

#[memoize]
fn get_arrangements(data_set: DataSet) -> u64 {
    // Handle the end cases where we're out of either springs or damaged sets.
    if data_set.spring_conditions.len() == 0 {
        if data_set.damaged_sets.len() == 0 {
            1 // No springs, and no damaged sets, this arrangement is good
        } else {
            0 // No springs, but we have damaged sets, this arrangement is bad
        }
    } else if data_set.damaged_sets.len() == 0 {
        if data_set.has_damaged_springs() {
            0  // We have no damaged sets, but do still have damaged springs, this arrangement is bad
        } else {
            1 // We have no damaged sets, and no damaged springs, this arrangement is good
        }
    } else {
        // If we get here then there are non-operational springs, and at least one damaged set left
        if data_set.can_fit_next_damaged_set() {
            let mut arrangements: u64 = 0;

            // Calculate potential arrangements where we assume the next spring is damaged
            if data_set.spring_conditions.len() == data_set.damaged_sets[0] as usize {
                // If the next damaged set fits perfectly, then we can assume it is operational
                arrangements += get_arrangements(DataSet {
                    spring_conditions: Vec::new(),
                    damaged_sets: data_set.damaged_sets[1..].to_vec(),
                }.trim_operational());
            } else {
                arrangements += get_arrangements(DataSet {
                    spring_conditions: data_set.spring_conditions[data_set.damaged_sets[0] as usize + 1..].to_vec(),
                    damaged_sets: data_set.damaged_sets[1..].to_vec(),
                }.trim_operational());
            }

            // If the next spring could be operational, also calculate where we assume it is
            if data_set.spring_conditions[0] == Condition::Unknown {
                arrangements += get_arrangements(DataSet {
                    spring_conditions: data_set.spring_conditions[1..].to_vec(),
                    damaged_sets: data_set.damaged_sets.clone(),
                }.trim_operational());
            }

            arrangements
        } else if data_set.spring_conditions[0] == Condition::Unknown {
            get_arrangements(DataSet {
                spring_conditions: data_set.spring_conditions[1..].to_vec(),
                damaged_sets: data_set.damaged_sets.clone(),
            }.trim_operational())
        } else {
            0
        }
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = &common::read_file_line_by_line(&input_file_path);

    let mut total_arrangements: u64 = 0;
    for line in lines {
        let data_set = DataSet::parse(line, !is_part_one);
        let arrangements = get_arrangements(data_set.trim_operational());
        total_arrangements += arrangements;
    }

    println!("Total arrangements: {}", total_arrangements);
}

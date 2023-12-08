use common::range::{Range, RangeSet};

#[derive(Debug, Clone, PartialEq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl Category {
    fn parse(string: &str) -> Category {
        match string {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => panic!("Unknown category: {}", string)
        }
    }
}

#[derive(Debug, Clone)]
struct ValueRangeSet {
    category: Category,
    ranges: RangeSet,
}

#[derive(Debug, Clone)]
struct ConversionRange {
    source_range: Range,
    modifier: i64,
}
impl ConversionRange {
    fn parse(string: &str) -> ConversionRange {
        let mut parts = string.split(" ");
        let destination_start = parts.next().unwrap().parse::<i64>().unwrap();
        let source_start = parts.next().unwrap().parse::<i64>().unwrap();
        let length = parts.next().unwrap().parse::<i64>().unwrap();
        ConversionRange {
            source_range: Range {
                start: source_start,
                end: source_start + length - 1
            },
            modifier: destination_start - source_start,
        }
    }
}

#[derive(Debug, Clone)]
struct Mapper {
    from_category: Category,
    to_category: Category,
    ranges: Vec<ConversionRange>
}

impl Mapper {
    fn map_ranges(&self, value_ranges: ValueRangeSet) -> ValueRangeSet {
        // Guard against mapping from the wrong category
        if value_ranges.category != self.from_category {
            panic!("Cannot map from {:?} to {:?}, expected {:?}",
                   value_ranges.category, self.to_category, self.from_category);
        }

        let mut ranges_to_convert = value_ranges.ranges.clone();
        let mut converted_ranges: Vec<Range> = Vec::new();
        for conversion_range in &self.ranges {
            let overlaps =
                ranges_to_convert.find_overlaps_with_range(&conversion_range.source_range);
            for overlap in overlaps.ranges {
                converted_ranges.push(Range {
                    start: overlap.start + conversion_range.modifier,
                    end: overlap.end + conversion_range.modifier,
                });
            }
            ranges_to_convert = ranges_to_convert.subtract_range(&conversion_range.source_range);
        }

        let mut unmapped_ranges = ranges_to_convert.ranges.clone();
        converted_ranges.append(&mut unmapped_ranges);

        ValueRangeSet {
            category: self.to_category.clone(),
            ranges: RangeSet {
                ranges: converted_ranges,
            }
        }
    }

    fn can_map(&self, value: &ValueRangeSet) -> bool {
        self.from_category == value.category
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let mut seeds: Vec<i64> = Vec::new();
    let mut mappers: Vec<Mapper> = Vec::new();
    let mut current_mapper: Option<Mapper> = None;
    for line in common::read_file_line_by_line(&input_file_path) {
        if line.starts_with("seeds: ") {
            seeds = line.replace("seeds: ", "").split(" ")
                .filter_map(|n| n.parse::<i64>().ok()).collect();
        } else if line.ends_with(" map:") {
            let parts: Vec<String> = line.replace(" map:", "")
                .split("-to-")
                .map(|s| s.to_string())
                .collect();
            let from_category = Category::parse(&parts[0]);
            let to_category = Category::parse(&parts[1]);
            current_mapper = Some(Mapper {
                from_category,
                to_category,
                ranges: Vec::new()
            });
        } else if line.len() > 0 {
            let range = ConversionRange::parse(&line);
            current_mapper.as_mut().unwrap().ranges.push(range);
        } else if current_mapper.is_some() {
            mappers.push(current_mapper.unwrap().clone());
            current_mapper = None;
        }
    }

    if current_mapper.is_some() {
        mappers.push(current_mapper.unwrap().clone());
    }

    let mut values = if is_part_one {
        ValueRangeSet {
            category: Category::Seed,
            ranges: RangeSet {
                ranges: seeds.iter().map(|x| Range { start: *x, end: *x }).collect()
            }
        }
    } else {
        ValueRangeSet {
            category: Category::Seed,
            ranges: RangeSet {
                ranges: seeds
                    .chunks(2)
                    .map(|x| Range { start: x[0], end: x[0] + x[1] - 1 })
                    .collect()
            }
        }
    };

    while values.category != Category::Location {
        for mapper in &mappers {
            if mapper.can_map(&values) {
                values = mapper.map_ranges(values);
                break;
            }
        }
    }

    let minimum_location_number =
        values.ranges.ranges.iter().map(|r| r.start).min().unwrap();
    println!("Minimum Location Number: {}", minimum_location_number);
}

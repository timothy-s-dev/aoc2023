#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn find_overlap(&self, other: &Range) -> Option<Range> {
        if self.end < other.start || self.start > other.end {
            return None;
        }
        let start = std::cmp::max(self.start, other.start);
        let end = std::cmp::min(self.end, other.end);
        Some(Range {
            start,
            end
        })
    }

    pub fn overlap_or_adjacent(&self, other: &Range) -> bool {
        self.end >= other.start - 1 && self.start <= other.end + 1
    }

    pub fn subtract(&self, other: &Range) -> RangeSet {
        if self.find_overlap(other).is_none() {
            return RangeSet {
                ranges: vec![self.clone()]
            };
        }
        let mut ranges: Vec<Range> = Vec::new();
        if self.start < other.start {
            ranges.push(Range {
                start: self.start,
                end: other.start - 1
            });
        }
        if self.end > other.end {
            ranges.push(Range {
                start: other.end + 1,
                end: self.end
            });
        }
        RangeSet {
            ranges
        }
    }
}

#[derive(Debug, Clone)]
pub struct RangeSet {
    pub ranges: Vec<Range>,
}
impl RangeSet {
    pub fn simplify(&self) -> RangeSet {
        let mut simplified_ranges: Vec<Range> = Vec::new();
        for range in &self.ranges {
            let mut found = false;
            for simplified_range in &mut simplified_ranges {
                if simplified_range.overlap_or_adjacent(range) {
                    simplified_range.start = std::cmp::min(range.start, simplified_range.start);
                    simplified_range.end = std::cmp::max(range.end, simplified_range.end);
                    found = true;
                    break;
                }
            }
            if !found {
                simplified_ranges.push(range.clone());
            }
        }
        RangeSet {
            ranges: simplified_ranges
        }
    }

    pub fn find_overlaps_with_range(&self, other: &Range) -> RangeSet {
        RangeSet {
            ranges: self.ranges.iter().filter_map(|r| r.find_overlap(other)).collect()
        }
    }

    pub fn subtract_range(&self, other: &Range) -> RangeSet {
        let mut subtracted_ranges: Vec<Range> = Vec::new();
        for range in &self.ranges {
            let mut subtracted_range = range.subtract(other);
            subtracted_ranges.append(&mut subtracted_range.ranges);
        }
        let result = RangeSet {
            ranges: subtracted_ranges
        };
        result.simplify()
    }

    pub fn subtract_ranges(&self, other: &RangeSet) -> RangeSet {
        let mut adjusted_range_set = self.clone();
        for other_range in &other.ranges {
            adjusted_range_set = adjusted_range_set.subtract_range(&other_range);
        }
        adjusted_range_set
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn find_overlap_when_none_returns_none() {
        let range1 = Range { start: 1, end: 2 };
        let range2 = Range { start: 3, end: 4 };
        let result = range1.find_overlap(&range2);
        assert_eq!(result, None);
    }

    #[test]
    fn find_overlap_when_partial_at_end_returns_overlap() {
        let range1 = Range { start: 1, end: 5 };
        let range2 = Range { start: 4, end: 7 };
        let result = range1.find_overlap(&range2);
        assert_eq!(result, Some(Range { start: 4, end: 5 }));
    }

    #[test]
    fn find_overlap_when_partial_at_start_returns_overlap() {
        let range1 = Range { start: 4, end: 7 };
        let range2 = Range { start: 1, end: 5 };
        let result = range1.find_overlap(&range2);
        assert_eq!(result, Some(Range { start: 4, end: 5 }));
    }

    #[test]
    fn find_overlap_when_contains_returns_overlap() {
        let range1 = Range { start: 1, end: 7 };
        let range2 = Range { start: 3, end: 5 };
        let result = range1.find_overlap(&range2);
        assert_eq!(result, Some(Range { start: 3, end: 5 }));
    }

    #[test]
    fn find_overlap_when_contained_returns_overlap() {
        let range1 = Range { start: 3, end: 5 };
        let range2 = Range { start: 1, end: 7 };
        let result = range1.find_overlap(&range2);
        assert_eq!(result, Some(Range { start: 3, end: 5 }));
    }

    #[test]
    fn subtract_when_no_overlap_returns_original_range() {
        let range1 = Range { start: 1, end: 2 };
        let range2 = Range { start: 3, end: 4 };
        let result = range1.subtract(&range2);
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(2, result.ranges[0].end);
    }

    #[test]
    fn subtract_when_partial_overlap_at_end_returns_new_range() {
        let range1 = Range { start: 1, end: 5 };
        let range2 = Range { start: 3, end: 7 };
        let result = range1.subtract(&range2);
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(2, result.ranges[0].end);
    }

    #[test]
    fn subtract_when_partial_overlap_at_start_returns_new_range() {
        let range1 = Range { start: 3, end: 7 };
        let range2 = Range { start: 1, end: 5 };
        let result = range1.subtract(&range2);
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(6, result.ranges[0].start);
        assert_eq!(7, result.ranges[0].end);
    }

    #[test]
    fn subtract_when_contains_overlap_returns_new_ranges() {
        let range1 = Range { start: 1, end: 7 };
        let range2 = Range { start: 3, end: 4 };
        let result = range1.subtract(&range2);
        assert_eq!(result.ranges.len(), 2);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(2, result.ranges[0].end);
        assert_eq!(5, result.ranges[1].start);
        assert_eq!(7, result.ranges[1].end);
    }

    #[test]
    fn subtract_when_contained_by_overlap_returns_new_ranges() {
        let range1 = Range { start: 3, end: 4 };
        let range2 = Range { start: 1, end: 7 };
        let result = range1.subtract(&range2);
        assert_eq!(result.ranges.len(), 0);
    }
}

#[cfg(test)]
mod range_set_tests {
    use super::*;

    #[test]
    fn simplify_when_adjacent_returns_combined() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 3, end: 4 },
            ]
        };
        let result = range_set.simplify();
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(4, result.ranges[0].end);
    }

    #[test]
    fn simplify_when_overlap_returns_combined() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 2, end: 7 },
            ]
        };
        let result = range_set.simplify();
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(7, result.ranges[0].end);
    }

    #[test]
    fn simplify_when_separate_returns_original() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let result = range_set.simplify();
        assert_eq!(result.ranges.len(), 2);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(2, result.ranges[0].end);
        assert_eq!(4, result.ranges[1].start);
        assert_eq!(7, result.ranges[1].end);
    }

    #[test]
    fn find_overlaps_with_range_when_none_returns_empty_vec() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let result = range_set.find_overlaps_with_range(&Range { start: 8, end: 9 });
        assert_eq!(result.ranges.len(), 0);
    }

    #[test]
    fn find_overlaps_with_range_when_one_returns_overlap() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let result = range_set.find_overlaps_with_range(&Range { start: 2, end: 3 });
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(2, result.ranges[0].start);
        assert_eq!(2, result.ranges[0].end);
    }

    #[test]
    fn find_overlaps_with_range_when_multiple_returns_all_overlaps() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let result = range_set.find_overlaps_with_range(&Range { start: 2, end: 5 });
        assert_eq!(result.ranges.len(), 2);
        assert_eq!(2, result.ranges[0].start);
        assert_eq!(2, result.ranges[0].end);
        assert_eq!(4, result.ranges[1].start);
        assert_eq!(5, result.ranges[1].end);
    }

    #[test]
    fn subtract_range_when_multiple_returns_all_remaining() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let result = range_set.subtract_range(&Range { start: 2, end: 5 });
        assert_eq!(result.ranges.len(), 2);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(1, result.ranges[0].end);
        assert_eq!(6, result.ranges[1].start);
        assert_eq!(7, result.ranges[1].end);
    }

    #[test]
    fn subtract_range_when_complete_overlap_removes() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let result = range_set.subtract_range(&Range { start: 1, end: 5 });
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(6, result.ranges[0].start);
        assert_eq!(7, result.ranges[0].end);
    }

    #[test]
    fn subtract_ranges_when_multiple_returns_all_remaining() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let other_range_set = RangeSet {
            ranges: vec![
                Range { start: 2, end: 3 },
                Range { start: 3, end: 5},
            ]
        };
        let result = range_set.subtract_ranges(&other_range_set);
        assert_eq!(result.ranges.len(), 2);
        assert_eq!(1, result.ranges[0].start);
        assert_eq!(1, result.ranges[0].end);
        assert_eq!(6, result.ranges[1].start);
        assert_eq!(7, result.ranges[1].end);
    }

    #[test]
    fn subtract_ranges_when_complete_overlap_removes() {
        let range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 2 },
                Range { start: 4, end: 7 },
            ]
        };
        let other_range_set = RangeSet {
            ranges: vec![
                Range { start: 1, end: 3 },
                Range { start: 3, end: 5},
                Range { start: 9, end: 10},
            ]
        };
        let result = range_set.subtract_ranges(&other_range_set);
        assert_eq!(result.ranges.len(), 1);
        assert_eq!(6, result.ranges[0].start);
        assert_eq!(7, result.ranges[0].end);
    }
}
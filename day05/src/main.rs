extern crate load_input;

use std::{collections::VecDeque, ops::Range};

#[derive(Clone)]
struct Entry {
    source: Range<u64>,
    destination: Range<u64>,
}

fn to_opt<T: Ord>(input: Range<T>) -> Option<Range<T>> {
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

impl Entry {
    fn translate(
        &self,
        input: Range<u64>,
    ) -> (Option<Range<u64>>, Option<Range<u64>>, Option<Range<u64>>) {
        use std::cmp::{max, min};
        let offset = self.destination.start.wrapping_sub(self.source.start);

        let prefix = input.start..min(input.end, self.source.start);
        let overlap = max(input.start, self.source.start)..min(input.end, self.source.end);
        let suffix = max(input.start, self.source.end)..input.end;

        let mapped_overlap = overlap.start.wrapping_add(offset)..overlap.end.wrapping_add(offset);

        let (prefix, overlap, suffix) = (to_opt(prefix), to_opt(mapped_overlap), to_opt(suffix));

        (prefix, overlap, suffix)
    }
}

fn get_nums(s: &str) -> Vec<Range<u64>> {
    let mut result = Vec::new();
    for candidate in s.split_whitespace() {
        if let Ok(n) = candidate.trim().parse::<u64>() {
            result.push(n..n + 1);
        }
    }

    result
}

fn get_mappings(s: &str) -> Vec<Entry> {
    let mut result = Vec::new();
    for line in s.lines().skip(1) {
        let nums = line
            .split_ascii_whitespace()
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        result.push(Entry {
            source: nums[1]..(nums[1] + nums[2]),
            destination: nums[0]..(nums[0] + nums[2]),
        });
    }

    result.sort_by_key(|e| e.source.start);
    result
}

fn translate(inputs: &[Range<u64>], mappings: &[Entry]) -> Vec<Range<u64>> {
    let mut result = Vec::new();
    let mut processing = inputs.iter().cloned().collect::<VecDeque<_>>();
    while let Some(mut input) = processing.pop_front() {
        for mapping in mappings {
            let (a, b, c) = mapping.translate(input);
            if let Some(a) = a {
                result.push(a);
            }

            if let Some(b) = b {
                result.push(b);
            }

            input = c.unwrap_or(0..0);
            if input.is_empty() {
                break;
            }
        }

        if !input.is_empty() {
            result.push(input);
        }
    }

    result
}

fn main() {
    let input = load_input::load_input();
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let seeds = get_nums(sections[0]);

    let mappings = sections[1..]
        .iter()
        .cloned()
        .map(get_mappings)
        .collect::<Vec<_>>();

    let mut nums = seeds.clone();
    for mapping in mappings.iter() {
        nums = translate(&nums, mapping);
    }

    println!("Part 1: {}", nums.iter().map(|r| r.start).min().unwrap());

    let mut nums = seeds
        .chunks(2)
        .map(|arr| arr[0].start..arr[0].start + arr[1].start)
        .collect::<Vec<_>>();

    for mapping in mappings.iter() {
        nums = translate(&nums, mapping);
    }

    println!("Part 2: {}", nums.iter().map(|r| r.start).min().unwrap());
}

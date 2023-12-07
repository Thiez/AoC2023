extern crate load_input;

use std::collections::{HashMap, HashSet};

struct Schematic {
    fields: Vec<Vec<char>>,
}

impl Schematic {
    fn get_char(&self, row: usize, col: usize) -> char {
        self.fields
            .get(row)
            .and_then(|r| r.get(col))
            .cloned()
            .unwrap_or('.')
    }

    fn read_num(&self, row: usize, mut col: usize) -> (usize, usize, u32) {
        while self.get_char(row, col).is_ascii_digit() {
            col = col.wrapping_sub(1);
        }

        col = col.wrapping_add(1);
        (
            row,
            col,
            self.fields[row][col..]
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .fold(0u32, |sum, c| 10 * sum + c.to_digit(10).unwrap()),
        )
    }
}

fn surrounding_coords(row: usize, col: usize) -> [(usize, usize); 8] {
    let (r1, r2, r3) = (row.wrapping_sub(1), row, row + 1);
    let (c1, c2, c3) = (col.wrapping_sub(1), col, col + 1);
    [
        (r1, c1),
        (r1, c2),
        (r1, c3),
        (r2, c1),
        (r2, c3),
        (r3, c1),
        (r3, c2),
        (r3, c3),
    ]
}

fn main() {
    let input = load_input::load_input();
    let schematic = Schematic {
        fields: input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    };

    let mut num_coords = HashSet::<(usize, usize, u32)>::new();
    let mut gear_coords = HashMap::<(usize, usize), HashSet<(usize, usize, u32)>>::new();
    for row in 0..schematic.fields.len() {
        let r = &schematic.fields[row];
        for col in 0..r.len() {
            let c = r[col];
            if c.is_ascii_digit() || c == '.' {
                continue;
            }

            for coords in surrounding_coords(row, col) {
                if schematic.get_char(coords.0, coords.1).is_ascii_digit() {
                    let num = schematic.read_num(coords.0, coords.1);
                    num_coords.insert(num);
                    if c == '*' {
                        gear_coords
                            .entry((row, col))
                            .or_insert(HashSet::new())
                            .insert(num);
                    }
                }
            }
        }
    }

    println!(
        "Part 1: {}",
        num_coords.iter().map(|&(_, _, n)| n).sum::<u32>()
    );

    println!(
        "Part 2: {}",
        gear_coords
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v
                .iter()
                .map(|&(_, _, n)| n)
                .fold(1, |product, n| product * n))
            .sum::<u32>()
    );
}

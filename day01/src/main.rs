extern crate load_input;

fn starts_with(s: &str, options: &[(&str, i32)]) -> Option<i32> {
    for &(opt, n) in options {
        if s.starts_with(opt) {
            return Some(n);
        }
    }

    None
}

fn first_and_last(mut s: &str, options: &[(&str, i32)]) -> Option<i32> {
    let mut result = None;
    while s != "" {
        result = match starts_with(s, options) {
            Some(n) => result.map(|(fst, _)| (fst, n)).or(Some((n, n))),
            _ => result,
        };
        s = &s[1..];
    }

    result.map(|(a, b)| 10 * a + b)
}

fn main() {
    let input = load_input::load_input();
    let p1_options = &[
        ("0", 0i32),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let s: i32 = input
        .lines()
        .flat_map(|line| first_and_last(line, p1_options))
        .sum();
    println!("Part 1: {}", s);

    let p2_options = &[
        ("0", 0i32),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let s: i32 = input
        .lines()
        .flat_map(|line| first_and_last(line, p2_options))
        .sum();
    println!("Part 2: {}", s);
}

extern crate load_input;

fn abc(a: i64, b: i64, c: i64) -> (i64, i64) {
    let (aa, bb, cc) = (a as f64, b as f64, c as f64);
    let d = bb * bb - 4.0 * aa * cc;
    assert!(d > 0.0);

    let (s1, s2) = ((-bb + d.sqrt()) / (aa + aa), (-bb - d.sqrt()) / (aa + aa));

    if s1 < s2 {
        (s1.ceil() as i64, s2.floor() as i64)
    } else {
        (s2.ceil() as i64, s1.floor() as i64)
    }
}

fn main() {
    let input = load_input::load_input();
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0].split_whitespace().flat_map(str::parse::<i64>);
    let distances = lines[1].split_whitespace().flat_map(str::parse::<i64>);
    let pairs = times.zip(distances).collect::<Vec<_>>();

    let part1 = pairs
        .iter()
        .map(|&(b, c)| abc(-1, b, -c - 1))
        .map(|(a, b)| b - a + 1)
        .product::<i64>();
    println!("Part 1: {}", part1);

    let pair = pairs.iter().fold((0i64, 0i64), |(a0, a1), &(b0, b1)| {
        (
            a0 * 10i64.pow(b0.ilog10() + 1) + b0,
            a1 * 10i64.pow(b1.ilog10() + 1) + b1,
        )
    });

    let (s1, s2) = abc(-1, pair.0, -pair.1 - 1);
    println!("Part 2: {}", s2 - s1 + 1);
}

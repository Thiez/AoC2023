extern crate load_input;

fn count_matches(winning: &[i32], have: &[i32]) -> usize {
    let mut matches = 0;
    for n in have {
        if winning.contains(n) {
            matches += 1;
        }
    }

    matches
}

fn score(matches: usize) -> usize {
    if matches == 0 {
        0
    } else {
        1 << (matches - 1)
    }
}

fn main() {
    let input = load_input::load_input();
    let parsed = input
        .lines()
        .flat_map(|s| s.split(':').skip(1))
        .map(|s| {
            s.split('|')
                .map(|nmbrs| {
                    nmbrs
                        .trim()
                        .split(' ')
                        .filter(|n| *n != "")
                        .map(|n| n.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let wins = parsed
        .iter()
        .map(|v| count_matches(&v[0], &v[1]))
        .collect::<Vec<_>>();

    let mut cards = Some(1usize)
        .into_iter()
        .cycle()
        .take(parsed.len())
        .collect::<Vec<_>>();

    println!("Part 1: {}", wins.iter().cloned().map(score).sum::<usize>());

    for i in 0..cards.len() {
        for offset in 0..wins[i] {
            cards[i + offset + 1] += cards[i];
        }
    }

    println!("Part 2: {}", cards.iter().sum::<usize>());
}

extern crate load_input;
use std::cmp::max;

struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    n: usize,
    hands: Vec<Hand>,
}

fn main() {
    let input = load_input::load_input();
    let mut games = Vec::new();
    for (n, line) in input.lines().enumerate() {
        let mut game = Game {
            n: n + 1,
            hands: Vec::new(),
        };
        let hands = line.split(':').skip(1).next().unwrap();
        for hand in hands.split(';') {
            let mut game_hand = Hand {
                red: 0,
                green: 0,
                blue: 0,
            };
            for col in hand.split(',').map(|s| s.trim()) {
                let ss = col.split_ascii_whitespace().collect::<Vec<&str>>();
                let amount = ss[0].parse::<usize>().unwrap();
                match ss[1] {
                    "red" => game_hand.red = amount,
                    "green" => game_hand.green = amount,
                    "blue" => game_hand.blue = amount,
                    _ => {}
                };
            }
            game.hands.push(game_hand);
        }

        games.push(game);
    }

    let part1_possible: usize = games
        .iter()
        .filter(|game| {
            game.hands
                .iter()
                .all(|hand| hand.red <= 12 && hand.green <= 13 && hand.blue <= 14)
        })
        .map(|game| game.n)
        .sum();
    println!("Part1: {}", part1_possible);

    let part2_sum: usize = games
        .iter()
        .map(|game| {
            game.hands.iter().fold(
                Hand {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |a, b| Hand {
                    red: max(a.red, b.red),
                    green: max(a.green, b.green),
                    blue: max(a.blue, b.blue),
                },
            )
        })
        .map(|hand| hand.red * hand.green * hand.blue)
        .sum();

    println!("Part2: {}", part2_sum);
}

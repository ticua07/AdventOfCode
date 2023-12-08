fn main() {
    let input = include_str!("./input.txt");

    process(input_parser(input));
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn process(race: Race) {
    let mut ways: Vec<u64> = vec![];

    let mut won = 0;
    for speed in 1..race.time {
        let mut position = 0;
        for _time in speed..race.time {
            position += speed;
        }
        if position > race.distance {
            won += 1
        }
    }
    ways.push(won);
    dbg!(ways.iter().product::<u64>());
}

fn input_parser(input: &str) -> Race {
    let (time, distance) = input.split_once("\r\n").unwrap();
    let time_iter = time
        .split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .skip(1)
        .collect::<Vec<&str>>()
        .join("");

    let distance_iter = distance
        .split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .skip(1)
        .collect::<Vec<&str>>()
        .join("");

    Race {
        time: time_iter.parse().expect("should be a number"),
        distance: distance_iter.parse().expect("should be a number"),
    }
}

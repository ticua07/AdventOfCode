use std::iter::zip;

fn main() {
    let input = include_str!("./input.txt");
    process(input_parser(input));
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn process(races: Vec<Race>) {
    let mut ways: Vec<u32> = vec![];
    for race in races {
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
        ways.push(won)
    }
    // dbg!(ways);
    dbg!(ways.iter().product::<u32>());
}

fn input_parser(input: &str) -> Vec<Race> {
    let (time, distance) = input.split_once("\r\n").unwrap();
    let time_iter = time
        .split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .skip(1)
        .collect::<Vec<&str>>();
    let distance_iter = distance
        .split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .skip(1)
        .collect::<Vec<&str>>();

    zip(time_iter, distance_iter)
        .map(|(t, d)| Race {
            time: t.parse::<u32>().expect("Should be a number"),
            distance: d.parse::<u32>().expect("Should be a number"),
        })
        .collect::<Vec<Race>>()
}

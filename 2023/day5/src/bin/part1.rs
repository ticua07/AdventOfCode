fn main() {
    let input = include_str!("input.txt");

    // input_parser(input);
    let (seeds, data) = input_parser(input);
    parse_numbers(seeds, data);
}

#[derive(Debug)]
struct SeedMap {
    destination: u64,
    source: u64,
    range: u64,
}

fn input_parser(input: &str) -> (Vec<u64>, Vec<Vec<SeedMap>>) {
    let mut data = input.split("\r\n\r\n").into_iter();
    let seed: Vec<u64> = data
        .next()
        .unwrap() // this should work
        .split(":")
        .last()
        .unwrap() // this should work
        .split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .map(|f| f.parse::<u64>().expect("couldn't convert number"))
        .collect();

    let remaining = data.collect::<Vec<&str>>();
    let ranges = remaining
        .iter()
        .map(|f| f.lines().skip(1).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let converted = ranges
        .iter()
        .map(|f| {
            f.iter()
                .map(|f| {
                    let a = f
                        .split_ascii_whitespace()
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|f| f.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    SeedMap {
                        destination: a[0],
                        source: a[1],
                        range: a[2],
                    }
                })
                .collect::<Vec<SeedMap>>()
        })
        .collect::<Vec<Vec<SeedMap>>>();

    (seed, converted)
}

// Define other category maps similarly

fn parse_numbers(seeds: Vec<u64>, input: Vec<Vec<SeedMap>>) {
    let mut current_seeds: Vec<u64> = seeds.clone();

    for block in input {
        let mut new_seeds: Vec<u64> = Vec::new();

        for seed in current_seeds {
            let mut found = false;

            for map in &block {
                // If the seed is between the range of the start of the source to the end
                // for example, if map.source is 50 and range is 2
                // seed 51 would be in the range and will be converted accordingly
                // this is because seed is bigger than 50 but less than 52
                // in this case the translation would end up like this
                // seed - map.source + map.destination = 51 - 50 + 98 = 99
                // and in fact, 51 translated is 99

                if seed >= map.source && seed < map.source + map.range {
                    new_seeds.push(seed - map.source + map.destination);
                    found = true;
                    break;
                }
            }

            if !found {
                new_seeds.push(seed);
            }
        }

        current_seeds = new_seeds;
    }

    dbg!(current_seeds.iter().min());
}

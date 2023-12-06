fn main() {
    let input = include_str!("input.txt");

    // input_parser(input);
    let (seeds, data) = input_parser(input);
    parse_numbers(get_range(seeds), data);
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

    // dest | src | range
}

// Define other category maps similarly
fn get_range(seeds: Vec<u64>) -> Vec<(u64, u64)> {
    let mut temp: Vec<(u64, u64)> = vec![];
    for idx in (0..seeds.len()).step_by(2) {
        let curr_elem = seeds[idx];
        let next_elem = *(seeds.get(idx + 1).unwrap_or(&0));
        temp.push((curr_elem, curr_elem + next_elem))
        // seeds_ranges.push(temp);
    }
    dbg!(&temp);
    temp
}

fn parse_numbers(seeds: Vec<(u64, u64)>, input: Vec<Vec<SeedMap>>) {
    let mut current_seeds: Vec<(u64, u64)> = seeds;

    for block in input {
        let mut new_seeds: Vec<(u64, u64)> = Vec::new();
        while current_seeds.len() > 0 {
            let (start, end) = current_seeds.pop().unwrap();
            let mut found = false;

            for map in &block {
                let overlap_s = std::cmp::max(start, map.source);
                let overlap_e = std::cmp::min(end, map.source + map.range);

                if overlap_s < overlap_e {
                    new_seeds.push((
                        overlap_s - map.source + map.destination,
                        overlap_e - map.source + map.destination,
                    ));
                    if overlap_s > start {
                        current_seeds.push((start, overlap_s));
                    }
                    if end > overlap_e {
                        current_seeds.push((overlap_e, end))
                    }

                    found = true;
                    break;
                }
            }
            if !found {
                new_seeds.push((start, end))
            }
            // if !found {
            // }
        }

        current_seeds = new_seeds;
    }

    dbg!(current_seeds.iter().min().unwrap().0);
}

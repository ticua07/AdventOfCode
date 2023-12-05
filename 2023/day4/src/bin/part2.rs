fn main() {
    let input = include_str!("./input.txt");
    part2(input);
}

fn part2(input: &str) {
    let cards: Vec<usize> = input.lines().map(parse_line).collect();
    let mut copies = vec![1; cards.len()];

    for (idx, &match_count) in cards.iter().enumerate() {
        for i in 0..match_count {
            copies[i + idx + 1] += copies[idx];
        }
    }

    println!("{:?}", copies.iter().sum::<usize>());
}

fn parse_line(line: &str) -> usize {
    let (_, mut data) = line.split_once(':').unwrap();

    // game_id_string = game_id_string.split_ascii_whitespace().last().unwrap();
    // let game_id = game_id_string.parse::<usize>().unwrap();
    data = data.trim();
    let scratchcard: Vec<&str> = data.split('|').map(|f| f.trim()).collect();

    let my_numbers = scratchcard[0]
        .split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .collect::<Vec<&str>>();

    let winning_numbers = scratchcard[1]
        .split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .collect::<Vec<&str>>();

    let repeated: Vec<&str> = my_numbers
        .into_iter()
        .filter(|f| winning_numbers.contains(f))
        .collect();

    return repeated.len();
}

fn main() {
    let input = include_str!("./debug-input.txt");
    part1(input);
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let (mut game_id, mut data) = line.split_once(":").unwrap();
        game_id = game_id.split_ascii_whitespace().last().unwrap();
        data = data.trim();
        let scratchcard: Vec<&str> = data.split("|").map(|f| f.trim()).collect();

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

        if repeated.len() != 0 {
            sum += 1 * u32::pow(2, i32::max(repeated.len() as i32 - 1, 0) as u32);
        }
    }

    println!("{sum}");
}

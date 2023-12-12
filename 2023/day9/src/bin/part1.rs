use itertools::{Itertools, Position};

fn main() {
    let input = include_str!("./input.txt");
    dbg!(parser(input));
}

fn parser(input: &str) -> i64 {
    let result: i64 = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let mut end_numbers: Vec<i64> = vec![]; // autocompleted numbers

            loop {
                if nums.iter().all(|num| num == &0) {
                    break;
                }
                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        match position {
                            Position::Last | Position::Only => end_numbers.push(*right),
                            _ => {}
                        };
                        right - left
                    })
                    .collect::<Vec<i64>>();
            }

            // dbg!(&end_numbers);
            let result = end_numbers.iter().fold(0, |acc, num| acc + num);

            result
        })
        .sum::<i64>();

    result
}

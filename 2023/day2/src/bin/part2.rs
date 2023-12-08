#![allow(unused_variables)]
fn main() {
    let input = include_str!("./input.txt");

    let mut sum = 0;

    for line in input.lines() {
        sum += part2(line);
    }
    println!("{sum}")
}

fn part2(input: &str) -> u32 {
    // Max number of:
    // red cubes -> 12
    // green cubes -> 13
    // blue cubes -> 14

    let mut red_cubes: u16 = 0;
    let mut green_cubes: u16 = 0;
    let mut blue_cubes: u16 = 0;

    let split_input = input.split(":").collect::<Vec<&str>>();

    let (game_id, values) = (
        split_input[0].split_ascii_whitespace().last().unwrap(),
        split_input[1].trim(),
    );

    for set in values.split(";") {
        for cubes in set.trim().split(",") {
            let split_cube = cubes.trim().split_ascii_whitespace().collect::<Vec<&str>>();
            let (amount, color) = (split_cube[0].parse::<u16>().unwrap(), split_cube[1]);

            // I thought that the sum of the cubes shouldn't be more than 12, 13 or 14
            // it isn't the sum, but showing X amount of cubes at once
            // Also it has to be > and not >=
            if color == "red" {
                red_cubes = std::cmp::max(red_cubes, amount);
            } else if color == "green" {
                green_cubes = std::cmp::max(green_cubes, amount);
            } else if color == "blue" {
                blue_cubes = std::cmp::max(blue_cubes, amount);
            }
        }
    }

    return (red_cubes * green_cubes * blue_cubes).into();
    // println!("-------------");
    // println!("{game_id} tiene:");
    // println!("{red_cubes} rojos:");
    // println!("{green_cubes} verdes:");
    // println!("{blue_cubes} azules:");
    // println!("-------------");

    // if red_cubes >= 12 || green_cubes >= 13 || blue_cubes >= 14 {
    //     return 0;
    // } else {
    //     return game_id.parse::<u32>().unwrap();
    // }
}

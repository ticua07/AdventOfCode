fn main() {
    let input = include_str!("./input.txt");

    let mut sum = 0;

    for line in input.lines() {
        sum += part1(line);
    }
    println!("{sum}")
}

fn part1(input: &str) -> u32 {
    // Max number of:
    // red cubes -> 12
    // green cubes -> 13
    // blue cubes -> 14

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
            if color == "red" && amount > 12 {
                return 0;
            } else if color == "green" && amount > 13 {
                return 0;
            } else if color == "blue" && amount > 14 {
                return 0;
            }
            // println!("{game_id}: tengo {amount} de {color}")
        }
    }

    return game_id.parse::<u32>().unwrap();
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

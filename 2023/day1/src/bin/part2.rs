#![allow(unused_variables, dead_code)]

fn main() {
    let input = include_str!("./input.txt");

    // println!("{}", part2("4nineeightseven2"))

    let mut sum: u32 = 0;

    for line in input.lines() {
        sum += part2(line);
    }

    println!("Sum of nums: {}", sum);
}

fn part2(input: &str) -> u32 {
    // Input isn't just numbers but letters too!
    let written_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let mut converted_input = input.to_string();

    for (idx, val) in written_numbers.iter().enumerate() {
        // Change all written numbers to normal numbers so we can reuse part 1's solution
        converted_input = converted_input.replace(val, (idx + 1).to_string().as_str())
    }

    let idx_first: Vec<Option<usize>> = written_numbers.iter().map(|val| input.find(val)).collect();

    let first_number = idx_first
        .iter()
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .min();

    let idx_second: Vec<Option<usize>> =
        written_numbers.iter().map(|val| input.rfind(val)).collect();

    let last_number = idx_second
        .iter()
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .max();

    // --- Explaining ---
    // Pretty much, My solution is get all the indixes of letters and numbers (following the same position as written_numbers)
    // Get the one nearest (or further if it's the last number) and then use that to
    // Get the index of the smallest number in the written_numbers vector (+1) and use that number

    let mut f_n = 0;
    let mut l_n = 0;
    if !idx_first.iter().all(|x| x.is_none()) {
        f_n = idx_first.iter().position(|f| f == &first_number).unwrap() + 1;
        if f_n > 9 {
            // Because the list is longer than 9, if the number is 9
            f_n -= 9
        }
    }
    if !idx_first.iter().all(|x| x.is_none()) {
        l_n = idx_second.iter().rposition(|f| f == &last_number).unwrap() + 1;
        if l_n > 9 {
            l_n -= 9
        }
    }
    format!("{:?}{:?}", f_n, l_n).parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = part2("two1nine");
        assert_eq!(num, 29);
    }

    #[test]
    fn test_2() {
        let num = part2("eightwothree");
        assert_eq!(num, 83);
    }

    #[test]
    fn test_3() {
        let num = part2("abcone2threexyz");
        assert_eq!(num, 13);
    }

    #[test]
    fn test_4() {
        let num = part2("4nineeightseven2");
        assert_eq!(num, 42);
    }
}

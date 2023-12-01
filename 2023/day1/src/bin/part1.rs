fn main() {
    let input = include_str!("./input.txt");
    let mut sum: u32 = 0;

    for line in input.lines() {
        sum += part1(line);
    }

    println!("Sum of nums: {}", sum);
}

fn part1(input: &str) -> u32 {
    let mut first_number: u32 = 0;
    let mut last_number: u32 = 0;

    'outer: for char in input.chars() {
        match char.to_digit(10) {
            Some(int) => {
                first_number = int;
                break 'outer;
            }
            None => continue,
        }
    }

    'outer: for char in input.chars().rev() {
        match char.to_digit(10) {
            Some(int) => {
                last_number = int;
                break 'outer;
            }
            None => continue,
        }
    }

    return format!("{}{}", first_number, last_number)
        .parse::<u32>()
        .expect("this should not error");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = part1("1abc2");
        assert_eq!(num, 12);
    }

    #[test]
    fn test_2() {
        let num = part1("pqr3stu8vwx");
        assert_eq!(num, 38);
    }

    #[test]
    fn test_3() {
        let num = part1("a1b2c3d4e5f");
        assert_eq!(num, 15);
    }

    #[test]
    fn test_4() {
        let num = part1("treb7uchet");
        assert_eq!(num, 77);
    }
}

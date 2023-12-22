fn process(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for c in input.chars() {
        sum += c as u32;
        sum = sum * 17;
        sum = sum % 256;
    }
    sum
}

fn main() {
    let input = include_str!("./input.txt");
    let data: u32 = input.split(",").map(process).sum();
    dbg!(data);
}

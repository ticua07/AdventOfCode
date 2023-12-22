use std::collections::HashMap;

fn hash(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for c in input.chars() {
        sum += c as u32;
        sum = sum * 17;
        sum = sum % 256;
    }
    sum
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Slot<'a> {
    code: &'a str,
    number: u32,
}

#[allow(unused_variables)]
fn process(input: &str) -> u32 {
    let actions: Vec<&str> = input.split(",").collect();
    let mut boxes: HashMap<u32, Vec<Slot>> = (0..=256).map(|f| (f, vec![])).collect();

    for action in actions {
        // if contains = means that action is append, otherwise we are deleting
        if action.contains("=") {
            let (code, number) = action.split_once("=").expect("should work");
            let code_hash = hash(code);

            let mut val = boxes.get(&code_hash).unwrap().to_owned();

            if let Some(index) = val
                .clone()
                .iter()
                .enumerate()
                .find(|(idx, f)| f.code == code)
            {
                val[index.0] = Slot {
                    code,
                    number: number.parse().unwrap(),
                };
            } else {
                val.push(Slot {
                    code,
                    number: number.parse().unwrap(),
                });
            }

            boxes.insert(code_hash, val.to_vec());
        } else {
            let (code, _) = action.split_once("-").expect("should work");
            let code_hash = hash(code);

            let mut val = boxes.get(&code_hash).unwrap().clone();

            if let Some(index) = val.iter().enumerate().find(|(idx, f)| f.code == code) {
                val.remove(index.0);
            }

            boxes.insert(code_hash, val);
        }
    }
    let mut sum = 0;
    for (key, vals) in boxes {
        for (idx, val) in vals.iter().enumerate() {
            // dbg!(key * (1 + idx as u32) * val.number);
            sum += (1 + key) * (1 + idx as u32) * val.number;
        }
    }
    sum
}

fn main() {
    let input = include_str!("./input.txt");
    dbg!(process(input));
}

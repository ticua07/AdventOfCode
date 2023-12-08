use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Left,
    Right,
}

fn input_parser(input: &str) -> (Vec<Instruction>, HashMap<&str, Node>) {
    let lines = input
        .lines()
        .filter(|f| !f.is_empty())
        .collect::<Vec<&str>>();
    let instructions = lines
        .first()
        .unwrap()
        .chars()
        .map(|f| match f {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!("should not happen!"),
        })
        .collect::<Vec<Instruction>>();

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for line in lines.iter().skip(1) {
        let (position, pointer) = line.split_once("=").unwrap();
        let (left, right) = pointer.split_once(",").unwrap();

        let node = Node {
            left: left.trim().trim_start_matches("("),
            right: right.trim_end_matches(")").trim(),
        };
        nodes.insert(position.trim(), node);
    }
    (instructions, nodes)
}

fn process(instructions: Vec<Instruction>, nodes: HashMap<&str, Node>, start_pos: &str) -> usize {
    let mut position = start_pos;
    // let start_positions = nodes
    //     .keys()
    //     .filter(|f| f.ends_with("A"))
    //     .map(|f| *f)
    //     .collect::<Vec<&str>>();

    let mut moves: usize = 0;

    'outer: loop {
        for instruction in &instructions {
            let next_node = match instruction {
                Instruction::Left => nodes.get(position).unwrap().left,
                Instruction::Right => nodes.get(position).unwrap().right,
            };
            // moves is added here instead of below the conditions, because break 'outer can mess with it
            moves += 1;
            if next_node.ends_with("Z") {
                break 'outer;
            } else {
                position = next_node
            }
        }
    }

    moves
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0; // LCM is not defined for zero
    }
    (a * b) / gcd(a, b)
}
fn main() {
    let input = include_str!("./input.txt");
    let (instructions, nodes) = input_parser(input);
    // dbg!(input_parser(input));
    let start_positions = nodes
        .keys()
        .filter(|f| f.ends_with("A"))
        .map(|f| *f)
        .collect::<Vec<&str>>();

    let steps = start_positions
        .iter()
        .map(|start| process(instructions.clone(), nodes.clone(), start))
        .collect::<Vec<usize>>();

    let result = steps.iter().fold(1, |acc, curr_v| lcm(acc, *curr_v));

    dbg!(result);
}

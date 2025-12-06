use std::io;

fn main() {
    let input: Vec<Vec<String>> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(' ')
                .filter(|e| !e.is_empty())
                .map(String::from)
                .collect()
        })
        .collect();

    let numbers: Vec<Vec<u64>> = input[0..input.len() - 1]
        .iter()
        .map(|l| l.iter().map(|e| e.as_str().parse().unwrap()).collect())
        .collect();

    let ops: Vec<_> = input[input.len() - 1]
        .iter()
        .map(|e| match e.as_str() {
            "+" => |a, b| a + b,
            "*" => |a, b| a * b,
            _ => panic!("Invalid operator"),
        })
        .collect();

    let res: u64 = ops
        .iter()
        .enumerate()
        .map(|(i, op)| {
            (0..numbers.len())
                .map(|j| numbers[j][i])
                .reduce(op)
                .unwrap()
        })
        .sum();
    println!("{res}");
}

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let numbers: Vec<Vec<u64>> = (0..lines[0].len())
        .map(|row| {
            let n = (0..lines.len() - 1)
                .map(|col| lines[col][row])
                .collect::<Vec<u8>>();
            str::from_utf8(n.as_slice()).unwrap().trim().parse().ok()
        })
        .fold(vec![vec![]], |mut v, o| {
            if let Some(n) = o {
                v.last_mut().unwrap().push(n);
            } else {
                v.push(vec![]);
            }
            v
        });

    let ops: Vec<fn(u64, u64) -> u64> = lines
        .last()
        .unwrap()
        .iter()
        .filter(|&&c| c != b' ')
        .map(|c| match c {
            b'+' => |a, b| a + b,
            b'*' => |a, b| a * b,
            _ => panic!("Invalid operator"),
        })
        .collect();

    assert_eq!(numbers.len(), ops.len());

    let res: u64 = ops
        .iter()
        .zip(numbers.iter())
        .map(|(&op, n)| n.iter().map(|&n| n).reduce(op).unwrap())
        .sum();

    println!("{res}");
}

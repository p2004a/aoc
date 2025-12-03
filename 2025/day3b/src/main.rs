use std::io;

fn max_in_slice<T: PartialOrd + Copy>(s: &[T]) -> usize {
    assert!(s.len() > 0);
    let mut idx = 0;
    for i in 1..s.len() {
        if &s[i] > &s[idx] {
            idx = i;
        }
    }
    return idx;
}

fn main() {
    let packs: Vec<Vec<u32>> = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut res: u64 = 0;
    for p in packs {
        let mut idx = 0;
        let mut joltage = 0;
        for d in (0..12).rev() {
            let max_idx = max_in_slice(&p[idx..p.len() - d]) + idx;
            joltage = joltage * 10 + (p[max_idx] as u64);
            idx = max_idx + 1;
        }
        res += joltage;
    }
    println!("{res}");
}

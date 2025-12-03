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

    let mut res = 0;
    for p in packs {
        let fst = max_in_slice(&p[..p.len() - 1]);
        let snd = max_in_slice(&p[fst + 1..]) + fst + 1;
        res += p[fst] * 10 + p[snd];
    }
    println!("{res}");
}

use std::io;

enum Rot {
    L(i32),
    R(i32),
}

impl Rot {
    fn from_str(s: &str) -> Option<Rot> {
        let n: i32 = s[1..].parse().ok()?;
        match &s[..1] {
            "R" => Some(Rot::R(n)),
            "L" => Some(Rot::L(n)),
            _ => None,
        }
    }
}

fn main() {
    let rotations: Vec<Rot> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| Rot::from_str(s).unwrap())
        .collect();

    let mut pos = 50;
    let mut zeros = 0;
    for r in rotations {
        pos = match r {
            Rot::L(n) => ((pos - n) % 100 + 100) % 100,
            Rot::R(n) => (pos + n) % 100,
        };
        if pos == 0 {
            zeros += 1
        }
    }
    println!("{zeros}");
}

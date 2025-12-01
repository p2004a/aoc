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
    let mut zeros = 0;
    io::stdin()
        .lines()
        .map(|l| Rot::from_str(l.unwrap().as_str()).unwrap())
        .fold(50, |pos, r| match r {
            Rot::L(n) => {
                let new_pos = (pos - 100) % 100 - n;
                zeros += new_pos / -100;
                (new_pos % 100 + 100) % 100
            }
            Rot::R(n) => {
                let new_pos = pos + n;
                zeros += new_pos / 100;
                new_pos % 100
            }
        });
    println!("{zeros}");
}

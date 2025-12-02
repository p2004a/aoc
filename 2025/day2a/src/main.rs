use std::io;

fn log10(mut n: u64) -> u64 {
    assert!(n > 0);
    let mut r = 0;
    while n > 0 {
        r += 1;
        n /= 10;
    }
    r
}

fn pow(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            r *= base;
        }
        base *= base;
        exp /= 2;
    }
    r
}

fn main() {
    let ranges: Vec<(u64, u64)> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .split(",")
        .map(|e| {
            let r: Vec<u64> = e.split("-").map(|n| n.parse().unwrap()).collect();
            assert!(r.len() == 2);
            (r[0], r[1])
        })
        .collect();

    let mut res = 0;
    for (from, to) in ranges {
        let mut half = pow(10, (log10(from) + 1) / 2);
        let upper = from / half;
        let lower = from % half;
        let mut n = if log10(from) % 2 == 1 {
            half / 10
        } else if lower <= upper {
            upper
        } else {
            upper + 1
        };
        // There is some closed form of this after computing the last element
        // deterministically that I'm not interested in finding.
        while n * half + n <= to {
            res += n * half + n;
            n += 1;
            if n >= half {
                half *= 10
            }
        }
    }
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log10() {
        assert_eq!(log10(1), 1);
        assert_eq!(log10(5), 1);
        assert_eq!(log10(11), 2);
        assert_eq!(log10(100), 3);
        assert_eq!(log10(1300), 4);
    }

    #[test]
    fn test_pow() {
        let mut res = 1;
        for exp in 0..30 {
            assert_eq!(pow(2, exp), res);
            res *= 2;
        }
    }
}

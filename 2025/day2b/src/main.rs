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

fn has_repeating(n: u64) -> bool {
    let digits = log10(n);
    let mut base = 1;
    'outer: for d in 1..=(digits / 2) {
        base *= 10;
        if digits % d == 0 {
            let num = n % base;
            let mut m = n;
            while m > 0 {
                if m % base != num {
                    continue 'outer;
                }
                m /= base;
            }
            return true;
        }
    }
    false
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
    // The input has only O(10^6) numbers so bruteforce
    for (from, to) in ranges {
        for n in from..=to {
            if has_repeating(n) {
                res += n;
            }
        }
    }
    println!("{res}");
}

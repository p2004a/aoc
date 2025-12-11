fn strip_brackets(s: &str) -> &str {
    &s[1..s.len() - 1]
}

fn main() {
    let mut res = 0;
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let parts: Vec<_> = line.split(' ').collect();
        let lights: u16 = strip_brackets(parts[0])
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("unexpected lights character"),
            })
            .rev()
            .fold(0, |acc, bit| (acc << 1) | bit);
        let buttons: Vec<u16> = parts[1..parts.len() - 1]
            .iter()
            .map(|p| {
                strip_brackets(p)
                    .split(',')
                    .map(|s| s.parse::<u16>().unwrap())
                    .fold(0, |acc, bit| acc | (1 << bit))
            })
            .collect();

        let mut min_bits = 1000;
        for mask in (0 as u16)..(1 << buttons.len()) {
            if mask.count_ones() >= min_bits {
                continue;
            }
            let mut res = 0;
            for (bit, &button) in buttons.iter().enumerate() {
                if (1 << bit) & mask != 0 {
                    res ^= button;
                }
            }
            if res == lights {
                min_bits = min_bits.min(mask.count_ones());
            }
        }
        res += min_bits;
    }
    println!("{res}");
}

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(i64, i64)> = ranges_str
        .split("\n")
        .map(|r| {
            let (f, t) = r.split_once("-").unwrap();
            (f.parse().unwrap(), t.parse().unwrap())
        })
        .collect();
    let ingredients: Vec<i64> = ingredients_str
        .trim()
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut brackets = ranges
        .iter()
        .flat_map(|&(from, to)| [(from, 1), (to + 1, -1)])
        .collect::<Vec<(i64, i64)>>();
    brackets.sort_by_key(|&(a, b)| (a, -b));
    let mut fresh_ranges = Vec::new();
    let mut opened = 0;
    for (pos, open) in brackets {
        if opened == 0 {
            fresh_ranges.push(pos)
        }
        opened += open;
        if opened == 0 {
            fresh_ranges.push(pos);
        }
    }

    let num_fresh = ingredients
        .iter()
        .filter(|&&i| fresh_ranges.partition_point(|&x| x < i) % 2 == 1)
        .count();
    println!("A num_fresh: {num_fresh}");

    let total_fresh_ids: i64 = fresh_ranges.chunks(2).map(|c| c[1] - c[0]).sum();
    println!("B total_fresh_ids: {total_fresh_ids}");
}

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let parts = input.trim().split("\n\n").collect::<Vec<_>>();
    let boxes = parts[0..parts.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let (n, b) = s.split_once(':').unwrap();
            assert_eq!(n.parse::<usize>().unwrap(), i);
            assert_eq!(b.trim().len(), 11);
            b.trim()
        })
        .collect::<Vec<_>>();
    let entries: Vec<((usize, usize), Vec<usize>)> = parts[parts.len() - 1]
        .split('\n')
        .map(|f| {
            let (size_str, counts_str) = f.split_once(':').unwrap();
            let (w, h) = size_str.split_once('x').unwrap();
            let counts: Vec<usize> = counts_str
                .trim()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(counts.len(), boxes.len());
            ((w.parse().unwrap(), h.parse().unwrap()), counts)
        })
        .collect();
    let boxes_parts: Vec<usize> = boxes
        .iter()
        .map(|&s| s.chars().filter(|&c| c == '#').count())
        .collect();

    let mut total_fit = 0;
    for ((w, h), counts) in entries.iter() {
        let min_fit_boxes = (w / 3) * (h / 3);
        let boxes_to_fit = counts.iter().sum::<usize>();
        let min_required_fields = counts
            .iter()
            .enumerate()
            .map(|(i, c)| boxes_parts[i] * c)
            .sum::<usize>();
        let fields = w * h;

        if boxes_to_fit <= min_fit_boxes {
            total_fit += 1;
        } else if min_required_fields > fields {
            total_fit += 0;
        } else {
            // cheeky, no cases like that in real input :)
            panic!("Unknown!");
        }
    }
    println!("{total_fit}");
}

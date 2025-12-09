type Point = (i64, i64);

fn area(from: Point, to: Point) -> i64 {
    ((from.0 - to.0).abs() + 1) * ((from.1 - to.1).abs() + 1)
}

fn main() {
    let points: Vec<Point> = std::io::stdin()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let (a, b) = line.split_once(',').unwrap();
            return (a.parse().unwrap(), b.parse().unwrap());
        })
        .collect();

    let mut max_area = 0;
    for a in 0..points.len() - 1 {
        for b in a + 1..points.len() {
            max_area = std::cmp::max(max_area, area(points[a], points[b]));
        }
    }
    println!("{max_area}");
}

// Note about solution: in the most possible generic case, this code does
// not cover all possible edge cases, but those edge cases do not exist
// in the input. I consider this property of input, the input is "nice".

use std::cmp::{max, min};

type Point = (i64, i64);

fn area(from: Point, to: Point) -> i64 {
    ((from.0 - to.0).abs() + 1) * ((from.1 - to.1).abs() + 1)
}

// rect must be sorted (bottom_left, top_right), as from get_rectangle
fn rectangle_intersect(rec: (Point, Point), l: (Point, Point)) -> bool {
    if l.0.0 == l.1.0 {
        // line vertical
        (rec.0.0 < l.0.0 && l.0.0 < rec.1.0)
            && !((l.0.1 <= rec.0.1 && l.1.1 <= rec.0.1) || (l.0.1 >= rec.1.1 && l.1.1 >= rec.1.1))
    } else {
        // line horizontal
        (rec.0.1 < l.0.1 && l.0.1 < rec.1.1)
            && !((l.0.0 <= rec.0.0 && l.1.0 <= rec.0.0) || (l.0.0 >= rec.1.0 && l.1.0 >= rec.1.0))
    }
}

fn get_rectangle(a: Point, b: Point) -> (Point, Point) {
    (
        (min(a.0, b.0), min(a.1, b.1)),
        (max(a.0, b.0), max(a.1, b.1)),
    )
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
    for ai in 0..points.len() - 1 {
        for bi in ai + 1..points.len() {
            let rect = get_rectangle(points[ai], points[bi]);
            let mut any_intersection = false;
            for i in 0..points.len() {
                let j = if i == 0 { points.len() - 1 } else { i - 1 };
                if rectangle_intersect(rect, (points[j], points[i])) {
                    any_intersection = true;
                    break;
                }
            }
            if !any_intersection {
                max_area = max(max_area, area(points[ai], points[bi]));
            }
        }
    }
    println!("{max_area}");
}

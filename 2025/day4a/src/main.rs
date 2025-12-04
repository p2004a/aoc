use std::io;

struct Grid<T> {
    d: Vec<T>,
    w: isize,
    h: isize,
}

impl<T: Copy + Clone> Grid<T> {
    fn new<F>(width: usize, height: usize, mut f: F) -> Grid<T>
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut d = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                d.push(f(x, y));
            }
        }
        Grid {
            w: width as isize,
            h: height as isize,
            d,
        }
    }

    fn get(&self, index: (isize, isize)) -> Option<T> {
        if index.0 < 0 || index.0 >= self.w || index.1 < 0 || index.1 >= self.h {
            None
        } else {
            Some(self.d[(index.1 * self.w + index.0) as usize])
        }
    }
}

fn main() {
    let input: Vec<Vec<bool>> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .split("\n")
        .map(|v| {
            v.as_bytes()
                .iter()
                .map(|c| match c {
                    b'.' => false,
                    b'@' => true,
                    _ => panic!("Unexpected char"),
                })
                .collect()
        })
        .collect();
    let grid = Grid::new(input[0].len(), input.len(), |x, y| input[y][x]);

    let mut free_rolls = 0;
    for y in 0..grid.h {
        for x in 0..grid.w {
            if grid.get((x, y)) != Some(true) {
                continue;
            }
            let mut around = 0;
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx != 0 || dy != 0 {
                        if grid.get((x + dx, y + dy)) == Some(true) {
                            around += 1;
                        }
                    }
                }
            }
            if around < 4 {
                free_rolls += 1;
            }
        }
    }
    println!("{free_rolls}");
}

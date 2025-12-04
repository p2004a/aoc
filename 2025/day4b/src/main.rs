use std::{collections::HashSet, io};

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

    fn in_bounds(&self, index: (isize, isize)) -> bool {
        return index.0 >= 0 && index.0 < self.w && index.1 >= 0 && index.1 < self.h;
    }

    fn get(&self, index: (isize, isize)) -> Option<T> {
        if self.in_bounds(index) {
            Some(self.d[(index.1 * self.w + index.0) as usize])
        } else {
            None
        }
    }

    fn set(&mut self, index: (isize, isize), val: T) {
        assert!(self.in_bounds(index));
        self.d[(index.1 * self.w + index.0) as usize] = val;
    }

    fn for_neighbors<F>(&self, pos: (isize, isize), mut f: F)
    where
        F: FnMut(T, (isize, isize)),
    {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let neighbor = (pos.0 + dx, pos.1 + dy);
                if (dx != 0 || dy != 0) && self.in_bounds(neighbor) {
                    f(self.get(neighbor).unwrap(), neighbor);
                }
            }
        }
    }
}

fn is_roll_removable(grid: &Grid<bool>, pos: (isize, isize)) -> bool {
    let mut around = 0;
    grid.for_neighbors(pos, |v, _| {
        if v {
            around += 1;
        }
    });
    return around < 4;
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
    let mut grid = Grid::new(input[0].len(), input.len(), |x, y| input[y][x]);

    let mut rolls: HashSet<(isize, isize)> = HashSet::new();
    for y in 0..grid.h {
        for x in 0..grid.w {
            if grid.get((x, y)) == Some(true) {
                rolls.insert((x, y));
            }
        }
    }

    let mut removed = 0;
    while !rolls.is_empty() {
        let mut new_rolls = HashSet::new();

        for roll_pos in rolls {
            if grid.get(roll_pos) == Some(true) && is_roll_removable(&grid, roll_pos) {
                grid.set(roll_pos, false);
                removed += 1;
                grid.for_neighbors(roll_pos, |_, pos| {
                    if grid.get(pos) == Some(true) {
                        new_rolls.insert(pos);
                    }
                });
            }
        }

        rolls = new_rolls;
    }
    println!("{removed}");
}

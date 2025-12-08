use std::collections::HashMap;

type Pos = (i64, i64, i64);
type Edge = (i64, usize, usize);

fn dist(a: &Pos, b: &Pos) -> i64 {
    let d = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
    d.0 * d.0 + d.1 * d.1 + d.2 * d.2
}

struct DisjoinSets {
    size: Vec<usize>,
    parent: Vec<usize>,
}

impl DisjoinSets {
    fn new(sets: usize) -> DisjoinSets {
        let mut size = Vec::new();
        size.resize(sets, 1);
        let mut parent = Vec::new();
        parent.resize(sets, 1);
        for i in 0..sets {
            parent[i] = i;
        }
        DisjoinSets { size, parent }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.parent[n] != n {
            self.parent[n] = self.find(self.parent[n]);
            self.parent[n]
        } else {
            n
        }
    }

    fn union(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            (a, b) = (b, a);
        }
        self.size[a] += self.size[b];
        self.parent[b] = a;
        true
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let nodes_to_link: u64 = args[1].parse().unwrap();

    let nodes: Vec<Pos> = std::io::stdin()
        .lines()
        .map(|l| {
            let pos = l
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(pos.len(), 3);
            (pos[0], pos[1], pos[2])
        })
        .collect();

    let mut edges: Vec<Edge> = Vec::new();
    edges.reserve((nodes.len() * (nodes.len() + 1)) / 2);
    for a in 0..nodes.len() {
        for b in (a + 1)..nodes.len() {
            edges.push((dist(&nodes[a], &nodes[b]), a, b));
        }
    }
    edges.sort();

    let mut ds = DisjoinSets::new(nodes.len());
    for (edge, _) in edges.iter().zip(0..nodes_to_link) {
        ds.union(edge.1, edge.2);
    }

    let mut set_sizes = HashMap::new();
    for i in 0..nodes.len() {
        let s = ds.find(i);
        set_sizes.insert(s, ds.size[s]);
    }

    let mut sizes = set_sizes.values().collect::<Vec<_>>();
    sizes.sort();
    sizes.reverse();

    println!("{}", sizes[0] * sizes[1] * sizes[2]);
}

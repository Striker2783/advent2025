use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
    path::Path,
};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p, 1000));
    println!("{}", solve2(&p));
}

fn parse(s: &str) -> Vec<(u64, u64, u64)> {
    s.lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn heap(v: &[(u64, u64, u64)]) -> BinaryHeap<Reverse<(u64, usize, usize)>> {
    let mut heap = BinaryHeap::new();
    for (i, &p1) in v.iter().enumerate() {
        for (j, &p2) in v.iter().enumerate().skip(i + 1) {
            let d = p1.0.abs_diff(p2.0).pow(2)
                + p1.1.abs_diff(p2.1).pow(2)
                + p1.2.abs_diff(p2.2).pow(2);
            heap.push(Reverse((d, i, j)));
        }
    }
    heap
}

#[derive(Debug)]
struct UnionFind {
    p: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
        }
    }
    fn sizes(&mut self) -> Vec<usize> {
        for i in 0..self.p.len() {
            self.update_nodes(i);
        }
        let mut map = HashMap::new();
        for &n in self.p.iter() {
            if let Some(n) = map.get_mut(&n) {
                *n += 1;
            } else {
                map.insert(n, 1);
            }
        }

        map.into_values().collect()
    }
    fn update_nodes(&mut self, mut i: usize) {
        let mut stack = Vec::new();
        while self.p[i] != i {
            stack.push(i);
            i = self.p[i];
        }
        while let Some(j) = stack.pop() {
            self.p[j] = i;
        }
    }
    pub fn connect(&mut self, x: usize, y: usize) {
        if self.same_set(x, y) {
            return;
        }
        let root = self.p[x];
        self.p[root] = y;
    }
    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.update_nodes(x);
        self.update_nodes(y);
        self.p[x] == self.p[y]
    }
}

fn solve1(v: &[(u64, u64, u64)], n: usize) -> u64 {
    let mut heap = heap(v);
    let mut union = UnionFind::new(v.len());
    for _ in 0..n {
        let Reverse((_, i, j)) = heap.pop().unwrap();
        union.connect(i, j);
    }
    let mut sizes = union.sizes();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).map(|n| *n as u64).product()
}

fn solve2(v: &[(u64, u64, u64)]) -> u64 {
    let mut heap = heap(v);
    let mut union = UnionFind::new(v.len());
    for _ in 0.. {
        let Reverse((_, i, j)) = heap.pop().unwrap();
        union.connect(i, j);
        let sizes = union.sizes();
        if sizes.len() == 1 {
            return v[i].0 * v[j].0;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::eight::{parse, solve1, solve2};

    const TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p, 10), 40);
        assert_eq!(solve2(&p), 25272);
    }
}

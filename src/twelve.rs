use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p.0, &p.1));
}

fn solve1(shapes: &[Vec<Vec<bool>>], regions: &[(u64, u64, Vec<u64>)]) -> u64 {
    let mut count = 0;
    let size = (shapes[0].len(), shapes[0][0].len());
    for r in regions {
        let sum: u64 = r.2.iter().cloned().sum();
        let total = r.0 * r.1 / size.0 as u64 / size.1 as u64;
        if total >= sum {
            count += 1;
        }
    }
    count
}

fn parse(s: &str) -> (Vec<Vec<Vec<bool>>>, Vec<(u64, u64, Vec<u64>)>) {
    let split1 = s.split("\n\n");
    let v: Vec<&str> = split1.collect();
    let shapes = v[0..(v.len() - 1)]
        .iter()
        .map(|shape| {
            shape
                .lines()
                .skip(1)
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();
    let regions = v
        .last()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split = l.split(": ");
            let mut region = split.next().unwrap().split('x');
            let (x, y) = (
                region.next().unwrap().parse().unwrap(),
                region.next().unwrap().parse().unwrap(),
            );
            let num = split
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            (x, y, num)
        })
        .collect();
    (shapes, regions)
}

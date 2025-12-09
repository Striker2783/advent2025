use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}

fn solve1(v: &[(u64, u64)]) -> u64 {
    let mut max = 0;
    for (i, p1) in v.iter().enumerate() {
        for p2 in v.iter().skip(i + 1) {
            let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
            max = max.max(area);
        }
    }
    max
}

fn contains(v: &[(u64, u64)], corners: &[(u64, u64); 4]) -> bool {
    'outer: for (&p1, &p2) in v.iter().zip(v.iter().skip(1).cycle()) {
        for (&c1, &c2) in corners.iter().zip(corners.iter().skip(1).cycle()) {
            if [p1, p2] == [c1, c2] {
                continue 'outer;
            } else if c1 == c2 {
                continue;
            }
            // if (p1.0 == p2.0 && c1.0 == c2.0) || (p1.1 == p2.1 && c1.1 == c2.1) {
            //     continue;
            // }
            if p1.0 == p2.0 {
                if (p1.1.min(p2.1)..=p1.1.max(p2.1)).contains(&c1.1) {
                    return false;
                }
            } else {
                if (p1.0.min(p2.0)..=p1.0.max(p2.0)).contains(&c1.0) {
                    return false;
                }
            }
        }
    }
    true
}

fn solve2(v: &[(u64, u64)]) -> u64 {
    let mut max = 0;
    for (i, &p1) in v.iter().enumerate() {
        for &p2 in v.iter().skip(i + 1) {
            let corners = [
                (p1.0.min(p2.0), p1.1.min(p2.1)),
                (p1.0.min(p2.0), p1.1.max(p2.1)),
                (p1.0.max(p2.0), p1.1.max(p2.1)),
                (p1.0.max(p2.0), p1.1.min(p2.1)),
            ];
            if !contains(v, &corners) {
                continue;
            }
            println!("{p1:?} {p2:?}");
            let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
            max = max.max(area);
        }
    }
    max
}

fn parse(s: &str) -> Vec<(u64, u64)> {
    s.lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::nine::{parse, solve1, solve2};

    const TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 50);
        assert_eq!(solve2(&p), 24);
    }
}

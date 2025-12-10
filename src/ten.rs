use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    // println!("{}", solve2(&p));
}

fn solve1(p: &[(Vec<bool>, Vec<Vec<usize>>)]) -> u64 {
    p.iter()
        .map(|config| {
            let mut queue = VecDeque::new();
            queue.push_back((vec![false; config.0.len()], 0));
            let mut set = HashSet::new();
            while let Some((mut lights, c)) = queue.pop_front() {
                if set.contains(&lights) {
                    continue;
                }
                set.insert(lights.clone());
                for switch in config.1.iter() {
                    let toggle = |v: &mut Vec<bool>| {
                        for &i in switch.iter() {
                            if i >= v.len() {
                                continue;
                            }
                            v[i] = !v[i];
                        }
                    };
                    toggle(&mut lights);
                    if lights == config.0 {
                        return c + 1;
                    }
                    queue.push_back((lights.clone(), c + 1));
                    toggle(&mut lights);
                }
            }
            unreachable!()
        })
        .sum()
}

fn parse(s: &str) -> Vec<(Vec<bool>, Vec<Vec<usize>>)> {
    let mut parsed = Vec::new();
    for l in s.lines() {
        let mut split = l.split(' ');
        let lights = split.next().unwrap();
        let lights = lights.as_bytes()[1..(lights.len() - 1)]
            .iter()
            .map(|&b| b == b'#')
            .collect();
        split.next_back();
        let v = split
            .map(|v| {
                v[1..(v.len() - 1)]
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();
        parsed.push((lights, v));
    }
    parsed
}

#[cfg(test)]
mod tests {
    use crate::ten::{parse, solve1};

    const TEST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 7);
    }
}

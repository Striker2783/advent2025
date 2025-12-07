use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}

fn solve1(v: &[Vec<u8>]) -> u32 {
    let mut v = v.to_vec();
    let mut count = 0;
    for i in 0..(v.len() - 1) {
        for j in 0..v[i].len() {
            let b = v[i][j];
            if b != b'S' {
                continue;
            }
            match v[i + 1][j] {
                b'.' => v[i + 1][j] = b'S',
                b'^' => {
                    count += 1;
                    v[i + 1][j - 1] = b'S';
                    v[i + 1][j + 1] = b'S';
                }
                _ => {}
            }
        }
    }
    count
}

fn solve2(v: &[Vec<u8>]) -> u64 {
    let mut v_count: Vec<Vec<_>> = (0..v.len())
        .map(|i| {
            (0..v[i].len())
                .map(|j| if v[i][j] == b'S' { 1 } else { 0 })
                .collect()
        })
        .collect();
    for i in 0..(v.len() - 1) {
        for j in 0..v[i].len() {
            match v[i + 1][j] {
                b'.' | b'S' => {
                    v_count[i + 1][j] += v_count[i][j];
                }
                b'^' => {
                    v_count[i + 1][j - 1] += v_count[i][j];
                    v_count[i + 1][j + 1] += v_count[i][j];
                }
                _ => {}
            }
        }
    }
    v_count.last().unwrap().iter().sum()
}

fn parse(s: &str) -> Vec<Vec<u8>> {
    s.lines().map(|l| l.as_bytes().to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use crate::seven::{parse, solve1, solve2};

    const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 21);
        assert_eq!(solve2(&p), 40);
    }
}

use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
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
            match v[i+1][j] {
                b'.' => v[i+1][j] = b'S',
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

fn parse(s: &str) -> Vec<Vec<u8>> {
    s.lines().map(|l| l.as_bytes().to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use crate::seven::{parse, solve1};

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
    }
}

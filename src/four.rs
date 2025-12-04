use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let v = parse(&s);
    println!("{}", solve1(&v));
}

fn solve1(v: &[Vec<bool>]) -> u32 {
    let mut count = 0;
    for (i, row) in v.iter().enumerate() {
        for (j, b) in row.iter().cloned().enumerate() {
            if !b {
                continue;
            }
            let mut rolls = 0;
            if i > 0 {
                if j > 0 && v[i - 1][j - 1] {
                    rolls += 1;
                }
                if v[i - 1][j] {
                    rolls += 1;
                }
                if j + 1 < v[0].len() && v[i - 1][j + 1] {
                    rolls += 1;
                }
            }
            if j > 0 && v[i][j - 1] {
                rolls += 1;
            }
            if j + 1 < v[0].len() && v[i][j + 1] {
                rolls += 1;
            }
            if i + 1 < v.len() {
                if j > 0 && v[i + 1][j - 1] {
                    rolls += 1;
                }
                if v[i + 1][j] {
                    rolls += 1;
                }
                if j + 1 < v.len() && v[i + 1][j + 1] {
                    rolls += 1;
                }
            }
            if rolls < 4 {
                count += 1;
            }
        }
    }
    count
}

fn parse(s: &str) -> Vec<Vec<bool>> {
    s.lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::four::{parse, solve1};

    const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    #[test]
    fn test_solve() {
        let v = parse(TEST);
        assert_eq!(solve1(&v), 13);
    }
}

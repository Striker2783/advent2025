use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let mut v = parse(&s);
    println!("{}", solve1(&v));
    println!("{}", solve2(&mut v));
}

fn can_remove(v: &[Vec<bool>], i: usize, j: usize) -> bool {
    let b = v[i][j];
    if !b {
        return false;
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
    rolls < 4
}

fn solve1(v: &[Vec<bool>]) -> u32 {
    let mut count = 0;
    for (i, row) in v.iter().enumerate() {
        for (j, _) in row.iter().cloned().enumerate() {
            if can_remove(v, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn recursive_remove(v: &mut [Vec<bool>], i: usize, j: usize) -> u32 {
    if !can_remove(v, i, j) {
        return 0;
    }
    let mut count = 1;
    v[i][j] = false;
    if i > 0 {
        if j > 0 {
            count += recursive_remove(v, i - 1, j - 1);
        }
        count += recursive_remove(v, i - 1, j);
        if j + 1 < v[0].len() {
            count += recursive_remove(v, i - 1, j + 1);
        }
    }
    if j > 0 {
        count += recursive_remove(v, i, j - 1);
    }
    if j + 1 < v[0].len() {
        count += recursive_remove(v, i, j + 1);
    }
    if i + 1 < v.len() {
        if j > 0 {
            count += recursive_remove(v, i + 1, j - 1);
        }
        count += recursive_remove(v, i + 1, j);
        if j + 1 < v[0].len() {
            count += recursive_remove(v, i + 1, j + 1);
        }
    }
    count
}

fn solve2(v: &mut [Vec<bool>]) -> u32 {
    let mut count = 0;
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            count += recursive_remove(v, i, j);
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
    use crate::four::{parse, solve1, solve2};

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
        let mut v = parse(TEST);
        assert_eq!(solve1(&v), 13);
        assert_eq!(solve2(&mut v), 43);
    }
}

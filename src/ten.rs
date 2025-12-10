use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
    vec,
};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}

fn solve1(p: &[MachineConfig]) -> u64 {
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

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    if a == 0 || b == 0 {
        return 0;
    }
    a * (b / gcd(a, b))
}

fn row_echelon(matrix: &mut Vec<Vec<i64>>) {
    let mut row = 0;
    for j in 0..matrix[0].len() {
        let pivot = (row..matrix.len()).find(|&i| matrix[i][j] != 0);
        if let Some(pivot) = pivot {
            matrix.swap(row, pivot);
            for i in 0..matrix.len() {
                if i == row || matrix[i][j] == 0 {
                    continue;
                }
                let lcm = lcm(matrix[row][j], matrix[i][j]);
                let temp_m = lcm / matrix[row][j];
                let temp: Vec<_> = matrix[row].iter().cloned().map(|n| n * temp_m).collect();
                let mult = lcm / matrix[i][j];
                matrix[i] = matrix[i]
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(i, n)| n * mult - temp[i])
                    .collect();
            }
            row += 1;
            if row >= matrix.len() {
                break;
            }
        }
    }
    for v in matrix.iter_mut() {
        let mut iterator = v.iter().copied().filter(|&n| n != 0);
        let mut gcd_n = 0;
        let mut first_neg = false;
        if let Some(r) = iterator.next() {
            gcd_n = r;
            if r < 0 {
                first_neg = true;
            }
            gcd_n = iterator.fold(gcd_n, gcd);
        }
        if gcd_n == 0 {
            continue;
        }
        v.iter_mut().for_each(|n| *n /= gcd_n);
        if first_neg {
            v.iter_mut().for_each(|n| *n *= -1);
        }
    }
    for i in (0..matrix.len()).rev() {
        if matrix[i].iter().all(|n| *n == 0) {
            matrix.remove(i);
        }
    }
}

fn free_vars(m: &[Vec<i64>]) -> Vec<usize> {
    let mut free = vec![true; m[0].len() - 1];
    m.iter().for_each(|v| {
        free[(0..(v.len() - 1)).find(|&i| v[i] != 0).unwrap()] = false;
    });
    free.iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(i) } else { None })
        .collect()
}

fn free_combinations(n: usize, mut f: impl FnMut(&[i64])) {
    fn helper(v: &mut Vec<i64>, n: usize, f: &mut impl FnMut(&[i64])) {
        if n == 0 {
            f(v);
            return;
        }
        for a in 0..=500 {
            v[n - 1] = a;
            helper(v, n - 1, f);
        }
    }
    helper(&mut vec![0; n], n, &mut f);
}

fn solve2(p: &[MachineConfig]) -> u64 {
    p.iter()
        .map(|config| {
            let mut matrix = vec![vec![0i64; config.1.len() + 1]; config.2.len()];
            (0..config.1.len()).for_each(|i| {
                for &j in config.1[i].iter() {
                    matrix[j][i] = 1;
                }
            });
            (0..config.2.len()).for_each(|i| {
                *matrix[i].last_mut().unwrap() = config.2[i] as i64;
            });
            row_echelon(&mut matrix);
            let free = free_vars(&matrix);

            if free.is_empty() {
                return matrix
                    .iter()
                    .map(|v| v.last().unwrap().unsigned_abs())
                    .sum();
            }
            let mut min = u64::MAX;
            free_combinations(free.len(), |frees| {
                let mut mult = vec![0; matrix[0].len() - 1];
                for (&n, &m) in frees.iter().zip(free.iter()) {
                    mult[m] = n;
                }
                let values: Option<Vec<i64>> = matrix
                    .iter()
                    .map(|row| {
                        let mut value = *row.last().unwrap()
                            - row
                                .iter()
                                .cloned()
                                .zip(mult.iter().cloned())
                                .map(|(r, m)| r * m)
                                .sum::<i64>();
                        let pivot = *row.iter().find(|&&n| n != 0).unwrap();
                        if pivot < 0 {
                            value *= -1;
                        }
                        let pivot = pivot.abs();
                        if value < 0 || value % pivot != 0 {
                            return None;
                        }
                        Some(value / pivot)
                    })
                    .collect();
                if let Some(v) = values {
                    let sum = v.iter().sum::<i64>() + frees.iter().sum::<i64>();
                    min = min.min(sum.unsigned_abs());
                }
            });
            if min > 10000 {
                println!("{free:?}");
                for v in matrix.iter() {
                    println!("{v:?}");
                }
                println!();
            }
            min
        })
        .sum()
}

type MachineConfig = (Vec<bool>, Vec<Vec<usize>>, Vec<u64>);

fn parse(s: &str) -> Vec<MachineConfig> {
    let mut parsed = Vec::new();
    for l in s.lines() {
        let mut split = l.split(' ');
        let lights = split.next().unwrap();
        let lights = lights.as_bytes()[1..(lights.len() - 1)]
            .iter()
            .map(|&b| b == b'#')
            .collect();
        let joltage = split.next_back().unwrap();
        let joltage = joltage[1..(joltage.len() - 1)]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let v = split
            .map(|v| {
                v[1..(v.len() - 1)]
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();
        parsed.push((lights, v, joltage));
    }
    parsed
}

#[cfg(test)]
mod tests {
    use crate::ten::{parse, solve1, solve2};

    const TEST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 7);
        assert_eq!(solve2(&p), 33);
    }
}

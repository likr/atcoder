use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [Chars; n],
    }
    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if c[i][j] == 's' {
                s = (i, j);
            }
            if c[i][j] == 'g' {
                g = (i, j);
            }
        }
    }

    let mut ok = -1.;
    let mut ng = 10.;
    let mut queue = vec![];
    let mut next_queue = vec![];
    let mut visited = HashSet::new();
    let mut passed = false;
    for _ in 0..50 {
        let l = (ok + ng) / 2.;
        queue.clear();
        queue.push(s);
        visited.clear();
        visited.insert(s);
        'find: for t in 1.. {
            if queue.is_empty() {
                ng = l;
                break;
            }
            next_queue.clear();
            for &(i, j) in &queue {
                if (i, j) == g {
                    passed = true;
                    ok = l;
                    break 'find;
                }
                for (dx, dy) in &[(0, 1), (2, 1), (1, 0), (1, 2)] {
                    if i + dy < 1 || n + 1 <= i + dy {
                        continue;
                    }
                    if j + dx < 1 || m + 1 <= j + dx {
                        continue;
                    }
                    let (i2, j2) = (i + dy - 1, j + dx - 1);
                    if !visited.contains(&(i2, j2))
                        && c[i2][j2] != '#'
                        && (c[i2][j2] == 'g'
                            || (c[i2][j2] as usize - '0' as usize) as f64 * (0.99f64).powi(t) >= l)
                    {
                        next_queue.push((i2, j2));
                        visited.insert((i2, j2));
                    }
                }
            }
            std::mem::swap(&mut queue, &mut next_queue);
        }
    }
    if passed {
        println!("{}", ok);
    } else {
        println!("-1");
    }
}

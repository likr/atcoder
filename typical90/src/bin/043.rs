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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        start: (Usize1, Usize1),
        goal: (Usize1, Usize1),
        s: [Chars; h],
    }
    let mut graph = vec![vec![vec![vec![]; w]; h]; 4];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                for k in 0..4 {
                    for l in 0..4 {
                        if k != l {
                            graph[k][i][j].push(((l, i, j), 1));
                        }
                    }
                }
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            if s[i][j] == '.' && s[i - 1][j] == '.' {
                graph[0][i][j].push(((0, i - 1, j), 0));
                graph[1][i - 1][j].push(((1, i, j), 0));
            }
        }
    }
    for i in 0..h {
        for j in 1..w {
            if s[i][j] == '.' && s[i][j - 1] == '.' {
                graph[2][i][j].push(((2, i, j - 1), 0));
                graph[3][i][j - 1].push(((3, i, j), 0));
            }
        }
    }
    let mut distance = vec![vec![vec![INF; w]; h]; 4];
    let mut queue = VecDeque::new();
    for k in 0..4 {
        distance[k][start.0][start.1] = 0;
        queue.push_back((k, start.0, start.1));
    }
    while let Some((k1, i1, j1)) = queue.pop_front() {
        for &((k2, i2, j2), c) in graph[k1][i1][j1].iter() {
            let d = distance[k1][i1][j1] + c;
            if d < distance[k2][i2][j2] {
                distance[k2][i2][j2] = d;
                if c == 0 {
                    queue.push_front((k2, i2, j2));
                } else {
                    queue.push_back((k2, i2, j2));
                }
            }
        }
    }
    println!(
        "{}",
        (0..4).map(|k| distance[k][goal.0][goal.1]).min().unwrap()
    );
}

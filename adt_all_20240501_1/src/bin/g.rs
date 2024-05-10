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
        n: usize,
        mut s: [Chars; n],
    }
    let mut p = vec![];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                p.push((i, j));
                s[i][j] = '.';
            }
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back((p[0].0, p[0].1, p[1].0, p[1].1));
    let mut distance = HashMap::new();
    distance.insert(*queue.front().unwrap(), 0);
    while let Some((i1, j1, i2, j2)) = queue.pop_front() {
        let d0 = distance[&(i1, j1, i2, j2)];
        if i1 == i2 && j1 == j2 {
            println!("{}", d0);
            return;
        }

        {
            let i1_next = if i1 > 0 && s[i1 - 1][j1] != '#' {
                i1 - 1
            } else {
                i1
            };
            let i2_next = if i2 > 0 && s[i2 - 1][j2] != '#' {
                i2 - 1
            } else {
                i2
            };
            let key = (i1_next, j1, i2_next, j2);
            if !distance.contains_key(&key) {
                queue.push_back(key);
                distance.insert(key, d0 + 1);
            }
        }
        {
            let i1_next = if i1 + 1 < n && s[i1 + 1][j1] != '#' {
                i1 + 1
            } else {
                i1
            };
            let i2_next = if i2 + 1 < n && s[i2 + 1][j2] != '#' {
                i2 + 1
            } else {
                i2
            };
            let key = (i1_next, j1, i2_next, j2);
            if !distance.contains_key(&key) {
                queue.push_back(key);
                distance.insert(key, d0 + 1);
            }
        }
        {
            let j1_next = if j1 > 0 && s[i1][j1 - 1] != '#' {
                j1 - 1
            } else {
                j1
            };
            let j2_next = if j2 > 0 && s[i2][j2 - 1] != '#' {
                j2 - 1
            } else {
                j2
            };
            let key = (i1, j1_next, i2, j2_next);
            if !distance.contains_key(&key) {
                queue.push_back(key);
                distance.insert(key, d0 + 1);
            }
        }
        {
            let j1_next = if j1 + 1 < n && s[i1][j1 + 1] != '#' {
                j1 + 1
            } else {
                j1
            };
            let j2_next = if j2 + 1 < n && s[i2][j2 + 1] != '#' {
                j2 + 1
            } else {
                j2
            };
            let key = (i1, j1_next, i2, j2_next);
            if !distance.contains_key(&key) {
                queue.push_back(key);
                distance.insert(key, d0 + 1);
            }
        }
    }
    println!("-1");
}

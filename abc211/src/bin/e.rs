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
        k: usize,
        s: [Chars; n],
    }
    let mut states = vec![HashSet::new()];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '.' {
                states[0].insert(1usize << (i * n + j));
            }
        }
    }
    for i in 1..k {
        let mut next = HashSet::new();
        for &x in states[i - 1].iter() {
            for j in 0..n * n {
                if x & 1 << j > 0 {
                    let a = j / n;
                    let b = j % n;
                    if a > 0 && s[a - 1][b] == '.' && x & 1 << ((a - 1) * n + b) == 0 {
                        next.insert(x | 1 << ((a - 1) * n + b));
                    }
                    if a + 1 < n && s[a + 1][b] == '.' && x & 1 << ((a + 1) * n + b) == 0 {
                        next.insert(x | 1 << ((a + 1) * n + b));
                    }
                    if b > 0 && s[a][b - 1] == '.' && x & 1 << (a * n + b - 1) == 0 {
                        next.insert(x | 1 << (a * n + b - 1));
                    }
                    if b + 1 < n && s[a][b + 1] == '.' && x & 1 << (a * n + b + 1) == 0 {
                        next.insert(x | 1 << (a * n + b + 1));
                    }
                }
            }
        }
        states.push(next);
    }
    println!("{}", states[k - 1].len());
}

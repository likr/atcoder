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
        t: [Chars; n],
    }
    for _ in 0..4 {
        let mut tmp = vec![vec!['.'; n]; n];
        for i in 0..n {
            for j in 0..n {
                tmp[i][j] = s[j][n - 1 - i];
            }
        }
        for i in 0..n {
            for j in 0..n {
                s[i][j] = tmp[i][j];
            }
        }
        let mut s_left = INF;
        let mut s_right = 0;
        let mut s_top = INF;
        let mut s_bottom = 0;
        let mut t_left = INF;
        let mut t_right = 0;
        let mut t_top = INF;
        let mut t_bottom = 0;
        for i in 0..n {
            for j in 0..n {
                if s[i][j] == '#' {
                    s_left = min(s_left, j);
                    s_right = max(s_right, j);
                    s_top = min(s_top, i);
                    s_bottom = max(s_bottom, i);
                }
                if t[i][j] == '#' {
                    t_left = min(t_left, j);
                    t_right = max(t_right, j);
                    t_top = min(t_top, i);
                    t_bottom = max(t_bottom, i);
                }
            }
        }
        let mut x = vec![vec!['.'; n]; n];
        for i in s_top..=s_bottom {
            for j in s_left..=s_right {
                x[i - s_top][j - s_left] = s[i][j];
            }
        }
        let mut y = vec![vec!['.'; n]; n];
        for i in t_top..=t_bottom {
            for j in t_left..=t_right {
                y[i - t_top][j - t_left] = t[i][j];
            }
        }
        // debug!(x);
        // debug!(y);
        if (0..n).all(|i| (0..n).all(|j| x[i][j] == y[i][j])) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

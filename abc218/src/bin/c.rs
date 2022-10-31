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
        s: [Chars; n],
        mut t: [Chars; n],
    }
    let s_y_offset = (0..n)
        .take_while(|&i| (0..n).all(|j| s[i][j] == '.'))
        .count();
    let s_x_offset = (0..n)
        .take_while(|&j| (0..n).all(|i| s[i][j] == '.'))
        .count();
    let mut s2 = vec![vec!['.'; n]; n];
    for i in s_y_offset..n {
        for j in s_x_offset..n {
            s2[i - s_y_offset][j - s_x_offset] = s[i][j];
        }
    }
    let mut t2 = vec![vec!['.'; n]; n];
    for _ in 0..4 {
        for i in 0..n {
            for j in 0..n {
                t2[i][j] = t[j][n - 1 - i];
            }
        }
        let t_y_offset = (0..n)
            .take_while(|&i| (0..n).all(|j| t2[i][j] == '.'))
            .count();
        let t_x_offset = (0..n)
            .take_while(|&j| (0..n).all(|i| t2[i][j] == '.'))
            .count();
        for i in 0..n {
            for j in 0..n {
                t[i][j] = '.';
            }
        }
        for i in t_y_offset..n {
            for j in t_x_offset..n {
                t[i - t_y_offset][j - t_x_offset] = t2[i][j];
            }
        }
        if s2 == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

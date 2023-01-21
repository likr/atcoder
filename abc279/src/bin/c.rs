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
        s: [Chars; h],
        t: [Chars; h],
    }
    let mut s_cols = HashMap::new();
    let mut t_cols = HashMap::new();
    for j in 0..w {
        let c = (0..h).map(|i| s[i][j]).collect::<String>();
        *s_cols.entry(c).or_insert(0) += 1;
        let c = (0..h).map(|i| t[i][j]).collect::<String>();
        *t_cols.entry(c).or_insert(0) += 1;
    }
    if s_cols == t_cols {
        println!("Yes");
    } else {
        println!("No");
    }
}

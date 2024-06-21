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
        d: usize,
        s: [Chars; n],
    }
    let mut f = vec!['o'; d];
    for i in 0..d {
        for j in 0..n {
            if s[j][i] == 'x' {
                f[i] = 'x';
            }
        }
    }
    let f = f.into_iter().collect::<String>();
    debug!(f);
    println!("{}", f.split("x").map(|si| si.len()).max().unwrap_or(0));
}

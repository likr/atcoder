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
const INF: i64 = std::i64::MAX / 4;
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
        c: i64,
        a: [[i64; w]; h],
    }
    let mut lower = vec![BTreeSet::new(); w];
    for i in 0..h {
        for j in 0..w {
            lower[j].insert((a[i][j] + c * i as i64, i, j));
        }
    }
    for j in 0..w {
        lower[j].insert((INF, 0, 0));
    }
    let mut result = INF;
    for i in 0..h {
        let mut right = BTreeSet::new();
        for j in 0..w {
            right.insert((
                lower[j].range(..).nth(0).unwrap().0 - c * i as i64 + c * j as i64,
                j,
            ));
        }
        let mut left = BTreeSet::new();
        left.insert((INF, 0));
        right.insert((INF, 0));
        for j in 0..w {
            right.remove(&(
                lower[j].range(..).nth(0).unwrap().0 - c * i as i64 + c * j as i64,
                j,
            ));
            lower[j].remove(&(a[i][j] + c * i as i64, i, j));
            result = min(
                result,
                a[i][j] + lower[j].range(..).nth(0).unwrap().0 - c * i as i64,
            );
            result = min(
                result,
                a[i][j] + left.range(..).nth(0).unwrap().0 - c * (w - j - 1) as i64,
            );
            result = min(
                result,
                a[i][j] + right.range(..).nth(0).unwrap().0 - c * j as i64,
            );
            lower[j].insert((a[i][j] + c * i as i64, i, j));
            left.insert((
                lower[j].range(..).nth(0).unwrap().0 - c * i as i64 + c * (w - 1 - j) as i64,
                j,
            ));
        }
        for j in 0..w {
            lower[j].remove(&(a[i][j] + c * i as i64, i, j));
        }
    }
    println!("{}", result);
}

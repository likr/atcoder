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
        m: usize,
        a: [Chars; 2 * n],
    }
    let mut win = vec![0; 2 * n];
    let mut indices = (0..2 * n).collect::<Vec<_>>();
    for j in 0..m {
        for i in (0..2 * n).step_by(2) {
            if (a[indices[i]][j] == 'G' && a[indices[i + 1]][j] == 'C')
                || (a[indices[i]][j] == 'C' && a[indices[i + 1]][j] == 'P')
                || (a[indices[i]][j] == 'P' && a[indices[i + 1]][j] == 'G')
            {
                win[indices[i]] += 1;
            }
            if (a[indices[i + 1]][j] == 'G' && a[indices[i]][j] == 'C')
                || (a[indices[i + 1]][j] == 'C' && a[indices[i]][j] == 'P')
                || (a[indices[i + 1]][j] == 'P' && a[indices[i]][j] == 'G')
            {
                win[indices[i + 1]] += 1;
            }
        }
        indices.sort_by_key(|&k| (Reverse(win[k]), k));
    }
    for i in 0..2 * n {
        println!("{}", indices[i] + 1);
    }
}

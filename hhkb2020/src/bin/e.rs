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
        s0: [Chars; h],
    }
    let mut s = vec![vec!['#'; w + 2]; h + 2];
    let mut empty_count = 0;
    for i in 1..=h {
        for j in 1..=w {
            s[i][j] = s0[i - 1][j - 1];
            if s[i][j] == '.' {
                empty_count += 1;
            }
        }
    }
    let mut h_count = vec![vec![0; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            if s[i][j - 1] == '#' && s[i][j] == '.' {
                let mut k = j;
                while s[i][k] == '.' {
                    k += 1;
                }
                for l in j..k {
                    h_count[i][l] = k - j;
                }
            }
        }
    }
    debug!(h_count);
    let mut v_count = vec![vec![0; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            if s[i - 1][j] == '#' && s[i][j] == '.' {
                let mut k = i;
                while s[k][j] == '.' {
                    k += 1;
                }
                for l in i..k {
                    v_count[l][j] = k - i;
                }
            }
        }
    }

    let mut pow2 = vec![1; empty_count + 1];
    for i in 1..=empty_count {
        pow2[i] = 2 * pow2[i - 1] % M;
    }

    let mut result = 0usize;
    for i in 1..=h {
        for j in 1..=w {
            if s[i][j] == '.' {
                let count = h_count[i][j] + v_count[i][j] - 1;
                debug!(i, j, count);
                let a = (pow2[count] + M - 1) % M;
                let b = pow2[empty_count - count];
                result = (result + a * b % M) % M;
            }
        }
    }
    println!("{}", result);
}

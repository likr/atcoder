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
        k: usize,
        s: (Usize1, Usize1),
        a: [[usize; w]; h],
    }
    let d = [(0, 1), (0, !0), (1, 0), (!0, 0)];
    let mut dp = vec![vec![vec![None; w]; h]; h * w + 1];
    dp[0][s.0][s.1] = Some(0);
    for l in 0..h * w {
        for i in 0..h {
            for j in 0..w {
                if let Some(v) = dp[l][i][j] {
                    for &(di, dj) in d.iter() {
                        let i2 = i.wrapping_add(di);
                        let j2 = j.wrapping_add(dj);
                        if i2 < h && j2 < w {
                            if let Some(w) = dp[l + 1][i2][j2] {
                                dp[l + 1][i2][j2] = Some(max(w, v + a[i2][j2]));
                            } else {
                                dp[l + 1][i2][j2] = Some(v + a[i2][j2]);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for l in 0..=min(k, h * w) {
        for i in 0..h {
            for j in 0..w {
                if let Some(v) = dp[l][i][j] {
                    ans = max(ans, v + a[i][j] * (k - l));
                }
            }
        }
    }
    println!("{}", ans);
}

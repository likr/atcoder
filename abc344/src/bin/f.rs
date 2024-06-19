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
        p: [[i64; n]; n],
        r: [[i64; n - 1]; n],
        d: [[i64; n]; n - 1],
    }
    let mut dp = vec![vec![vec![None; n]; n]; n * n];
    dp[0][0][0] = Some((0, Reverse(0)));
    for i in 0..n {
        for j in 0..n {
            for k in 0..n * n {
                if let Some((c, Reverse(q))) = dp[k][i][j] {
                    let mut update = |k: usize, i: usize, j: usize, c: i64, d: i64| {
                        if let Some((c0, Reverse(d0))) = dp[k][i][j] {
                            dp[k][i][j] = Some(min((c, Reverse(d)), (c0, Reverse(d0))));
                        } else {
                            dp[k][i][j] = Some((c, Reverse(d)));
                        }
                    };
                    let pk = p[k / n][k % n];
                    if i + 1 < n {
                        if d[i][j] <= q {
                            let c2 = c + 1;
                            let q2 = q - d[i][j];
                            update(k, i + 1, j, c2, q2);
                            update((i + 1) * n + j, i + 1, j, c2, q2);
                        } else {
                            let c2 = (d[i][j] - q + pk - 1) / pk;
                            let q2 = c2 * pk + q - d[i][j];
                            update(k, i + 1, j, c + c2 + 1, q2);
                            update((i + 1) * n + j, i + 1, j, c + c2 + 1, q2);
                        }
                    }
                    if j + 1 < n {
                        if r[i][j] <= q {
                            let c2 = c + 1;
                            let q2 = q - r[i][j];
                            update(k, i, j + 1, c2, q2);
                            update(i * n + j + 1, i, j + 1, c2, q2);
                        } else {
                            let c2 = (r[i][j] - q + pk - 1) / pk;
                            let q2 = c2 * pk + q - r[i][j];
                            update(k, i, j + 1, c + c2 + 1, q2);
                            update(i * n + j + 1, i, j + 1, c + c2 + 1, q2);
                        }
                    }
                }
            }
        }
    }
    let mut ans = INF as i64;
    for k in 0..n * n {
        if let Some((c, _)) = dp[k][n - 1][n - 1] {
            ans = min(ans, c);
        }
    }
    println!("{}", ans);
}

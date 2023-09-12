use ac_library::*;
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
        a: [usize; n],
    }
    let f = ModInt998244353::new;
    let mut ans = f(n);
    for d in 2..=n {
        let mut dp = vec![vec![vec![f(0); d]; d + 1]; n + 1];
        dp[0][0][0] = f(1);
        for i in 0..n {
            for j in 0..=d {
                for k in 0..d {
                    dp[i + 1][j][k] = dp[i + 1][j][k] + dp[i][j][k];
                    if j + 1 <= d {
                        dp[i + 1][j + 1][(k + a[i]) % d] =
                            dp[i + 1][j + 1][(k + a[i]) % d] + dp[i][j][k];
                    }
                }
            }
        }

        ans = ans + dp[n][d][0];
    }
    println!("{}", ans);
}

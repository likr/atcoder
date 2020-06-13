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

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(Chars, usize); m],
    }
    let sc = sc
        .into_iter()
        .map(|(s, c)| {
            let mut t = 0;
            for i in 0..n {
                if s[i] == 'Y' {
                    t |= 1 << i;
                }
            }
            (t, c)
        })
        .collect::<Vec<_>>();
    let mut dp = vec![INF; 2usize.pow(n as u32)];
    dp[0] = 0;
    for &(si, ti) in &sc {
        for j in 0..dp.len() {
            dp[j | si] = min(dp[j | si], dp[j] + ti);
        }
    }
    if dp[dp.len() - 1] == INF {
        println!("-1");
    } else {
        println!("{}", dp[dp.len() - 1]);
    }
}

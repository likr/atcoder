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
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }
    let mut dp = vec![vec![-INF; w + 1]; n];
    {
        for i in 0..n {
            dp[i][0] = 0;
        }
        let (li, ri, vi) = lrv[0];
        for j in li..=ri {
            dp[0][j] = vi;
        }
    }
    let mut heap = BinaryHeap::new();
    for i in 1..n {
        let (li, ri, vi) = lrv[i];
        heap.clear();
        for j in 0..=w {
            if li <= j {
                heap.push((dp[i - 1][j - li], j - li));
            }
            while let Some(&(v, c)) = heap.peek() {
                if c + ri < j || v < 0 {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some(&(v, _)) = heap.peek() {
                dp[i][j] = max(dp[i - 1][j], v + vi);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    if dp[n - 1][w] == -INF {
        println!("-1");
    } else {
        println!("{}", dp[n - 1][w]);
    }
}

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
        k: usize,
        s: [Chars; n],
    }

    let s = s
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|d| d as usize - '0' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count = vec![0; 10];
    for x in (1..=min(n, m)).rev() {
        for i in 0..=n - x {
            for j in 0..=m - x {
                for d in 0..10 {
                    count[d] = 0;
                }
                for di in 0..x {
                    for dj in 0..x {
                        count[s[i + di][j + dj]] += 1;
                    }
                }
                let max_count = count.iter().max().unwrap();
                if max_count + k >= x * x {
                    println!("{}", x);
                    return;
                }
            }
        }
    }
}

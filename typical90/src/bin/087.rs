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

fn solve1(n: usize, p: usize, k: usize, a: &Vec<Vec<i64>>) -> usize {
    let mut d = vec![vec![0usize; n]; n];
    let mut ok = 0;
    let mut ng = INF;
    while ng - ok > 1 {
        let m = (ng + ok) / 2;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == -1 {
                    d[i][j] = m;
                } else {
                    d[i][j] = a[i][j] as usize;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
                }
            }
        }
        let mut count = 0;
        for i in 0..n {
            for j in 0..i {
                if d[i][j] <= p {
                    count += 1;
                }
            }
        }
        if count >= k {
            ok = m;
        } else {
            ng = m;
        }
    }
    ok
}

fn solve2(n: usize, p: usize, k: usize, a: &Vec<Vec<i64>>) -> usize {
    let mut d = vec![vec![0usize; n]; n];
    let mut ng = 0;
    let mut ok = INF;
    while ok - ng > 1 {
        let m = (ng + ok) / 2;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == -1 {
                    d[i][j] = m;
                } else {
                    d[i][j] = a[i][j] as usize;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
                }
            }
        }
        let mut count = 0;
        for i in 0..n {
            for j in 0..i {
                if d[i][j] <= p {
                    count += 1;
                }
            }
        }
        if count <= k {
            ok = m;
        } else {
            ng = m;
        }
    }
    ng
}

fn main() {
    input! {
        n: usize,
        p: usize,
        k: usize,
        a: [[i64; n]; n],
    }
    let u = solve1(n, p, k, &a);
    let l = solve2(n, p, k, &a);
    if u + 1 == INF {
        if l + 1 == INF {
            println!("0");
        } else {
            println!("Infinity");
        }
    } else {
        println!("{}", u - l);
    }
}

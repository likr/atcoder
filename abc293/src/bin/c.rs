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
        a: [[usize; w]; h],
    }
    let n = h + w - 2;
    let x = (0..(1 << n))
        .filter(|&xi| {
            let mut count = 0;
            for i in 0..n {
                if xi & 1 << i > 0 {
                    count += 1;
                }
            }
            count == w - 1
        })
        .collect::<Vec<_>>();
    let mut result = 0;
    'outer: for &xi in x.iter() {
        let mut i = 0;
        let mut j = 0;
        let mut visited = HashSet::new();
        visited.insert(a[i][j]);
        for k in 0..n {
            if xi & 1 << k > 0 {
                j += 1;
            } else {
                i += 1;
            }
            if visited.contains(&a[i][j]) {
                continue 'outer;
            }
            visited.insert(a[i][j]);
        }
        result += 1;
    }
    println!("{}", result);
}

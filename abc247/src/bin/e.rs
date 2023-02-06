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
        x: usize,
        y: usize,
        a: [usize; n],
    }
    let mut s = vec![];
    for i in 0..n {
        if a[i] > x || y > a[i] {
            s.push(4usize);
        } else if x == y {
            s.push(1);
        } else if a[i] == x {
            s.push(2)
        } else if a[i] == y {
            s.push(0)
        } else {
            s.push(1);
        }
    }
    let segments = s
        .split(|&si| si == 4)
        .map(|si| si.to_vec())
        .filter(|si| si.len() > 0)
        .collect::<Vec<_>>();
    debug!(segments);
    let m = segments.len();
    if x == y {
        let mut result = 0;
        for i in 0..m {
            let s = &segments[i];
            let n = s.len();
            result += n * (n + 1) / 2;
        }
        println!("{}", result);
        return;
    }
    let mut result = 0;
    for k in 0..m {
        let segment = &segments[k];
        let n = segment.len();
        let mut s = n * (n + 1) / 2;
        let mut offset = 0;
        let mut count = vec![0; 3];
        for i in 0..n {
            while offset < n && (count[0] == 0 || count[2] == 0) {
                count[segment[offset]] += 1;
                offset += 1;
            }
            let l = if count[0] == 0 || count[2] == 0 {
                offset - i
            } else {
                offset - i - 1
            };
            debug!(l);
            s -= l;
            count[segment[i]] -= 1;
        }
        result += s;
    }
    println!("{}", result);
}

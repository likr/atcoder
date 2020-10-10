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
        s: Chars,
    }
    let n = s.len();
    let mut segments = vec![0];
    for i in 1..n {
        if s[i - 1] == 'L' && s[i] == 'R' {
            segments.push(i);
        }
    }
    segments.push(n);
    debug!(segments);

    let mut result = vec![0; n];
    for k in 1..segments.len() {
        let left = segments[k - 1];
        let right = segments[k];
        let mut odd = 0;
        let mut even = 0;
        for i in left..right {
            if i % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        for i in left + 1..right {
            if s[i - 1] == 'R' && s[i] == 'L' {
                if i % 2 == 0 {
                    result[i - 1] = odd;
                    result[i] = even;
                } else {
                    result[i] = odd;
                    result[i - 1] = even;
                }
            }
        }
    }

    for i in 0..n {
        println!("{}", result[i]);
    }
}

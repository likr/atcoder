use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        q: usize,
        s: Chars,
        td: [(char, char); q],
    }
    let left_max = {
        let indices = (0..n).collect::<Vec<_>>();
        indices.lower_bound_by(|&k| {
            let mut k = k;
            for i in 0..q {
                let (ti, di) = td[i];
                if s[k] == ti {
                    if di == 'L' {
                        if k == 0 {
                            return Ordering::Less;
                        }
                        k -= 1;
                    } else {
                        if k == n - 1 {
                            return Ordering::Greater;
                        }
                        k += 1;
                    }
                }
            }
            Ordering::Greater
        })
    };
    let right_min = {
        let indices = (0..n).rev().collect::<Vec<_>>();
        n - indices.lower_bound_by(|&k| {
            let mut k = k;
            for i in 0..q {
                let (ti, di) = td[i];
                if s[k] == ti {
                    if di == 'L' {
                        if k == 0 {
                            return Ordering::Greater;
                        }
                        k -= 1;
                    } else {
                        if k == n - 1 {
                            return Ordering::Less;
                        }
                        k += 1;
                    }
                }
            }
            Ordering::Greater
        })
    };
    debug!(left_max, right_min);
    if right_min < left_max {
        println!("0");
    } else {
        println!("{}", right_min - left_max);
    }
}

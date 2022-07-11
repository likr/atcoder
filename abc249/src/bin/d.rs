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
    let mut count = HashMap::new();
    for &ai in a.iter() {
        *count.entry(ai).or_insert(0) += 1;
    }
    let mut result = 0usize;
    for &ai in a.iter() {
        for d in 1.. {
            if d * d > ai {
                break;
            }
            if ai % d == 0 {
                if let Some(&c1) = count.get(&d) {
                    if let Some(&c2) = count.get(&(ai / d)) {
                        if d == ai / d {
                            result += c1 * c2;
                        } else {
                            result += 2 * c1 * c2;
                        }
                    }
                }
            }
        }
    }
    println!("{}", result);
}

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
        q: usize,
    }
    let mut values = BTreeSet::new();
    for i in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        match t {
            1 => {
                values.insert((x, i));
            }
            2 => {
                input! {
                    k: Usize1,
                }
                if let Some((y, _)) = values.range(..=(x, INF)).rev().nth(k) {
                    println!("{}", y);
                } else {
                    println!("-1");
                }
            }
            _ => {
                input! {
                    k: Usize1,
                }
                if let Some((y, _)) = values.range((x, 0)..).nth(k) {
                    println!("{}", y);
                } else {
                    println!("-1");
                }
            }
        }
    }
}

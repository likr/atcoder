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
        q: usize,
    }
    let mut base = 0;
    let mut a = a.into_iter().enumerate().collect::<HashMap<_, _>>();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                a.clear();
                base = x;
            }
            2 => {
                input! {
                    i: Usize1,
                    x: usize,
                }
                *a.entry(i).or_insert(0) += x;
            }
            _ => {
                input! {
                    i: Usize1,
                }
                if let Some(v) = a.get(&i) {
                    println!("{}", v + base);
                } else {
                    println!("{}", base);
                }
            }
        }
    }
}

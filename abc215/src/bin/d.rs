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
        a: [usize; n],
    }
    let mut factors = HashSet::new();
    for &ai in a.iter() {
        for x in 1.. {
            if x * x > ai {
                break;
            }
            if ai % x == 0 {
                factors.insert(x);
                factors.insert(ai / x);
            }
        }
    }
    factors.remove(&1);
    let mut ok = vec![true; m + 1];
    ok[0] = false;
    for &x in factors.iter() {
        for i in 1.. {
            if x * i > m {
                break;
            }
            ok[x * i] = false;
        }
    }
    let count = ok.iter().filter(|&&f| f).count();
    println!("{}", count);
    for i in 1..=m {
        if ok[i] {
            println!("{}", i);
        }
    }
}

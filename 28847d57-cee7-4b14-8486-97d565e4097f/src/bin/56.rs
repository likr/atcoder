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
        p: usize,
    }
    let mut q = p;
    let mut factors = HashMap::new();
    for d in 2.. {
        if d * d > p {
            break;
        }
        while q % d == 0 {
            q /= d;
            *factors.entry(d).or_insert(0) += 1;
        }
    }
    if q > 1 {
        *factors.entry(q).or_insert(0) += 1;
    }
    let mut ans = 1usize;
    for (&d, &c) in factors.iter() {
        ans *= d.pow((c / n) as u32);
    }
    println!("{}", ans);
}

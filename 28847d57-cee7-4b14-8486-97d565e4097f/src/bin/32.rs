use ac_library::*;
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

fn f(x: usize) -> ModInt1000000007 {
    let m = ModInt1000000007::new;
    let mut c = 1;
    let mut base = 1;
    let mut s = m(0);
    loop {
        if x < base * 10 {
            s += m(c) * (m(x) - m(base) + m(1)) * (m(base) + m(x)) / m(2);
            break;
        } else {
            s += m(c) * (m(base) * m(10) - m(base)) * (m(base) + m(base) * m(10) - m(1)) / m(2);
        }
        c += 1;
        base *= 10;
    }
    s
}

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    println!("{}", (f(r) - f(l - 1)).val());
}

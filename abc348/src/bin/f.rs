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
        a: [[u16; m]; n],
    }
    let mut ans = 0;
    for j in 0..n {
        for i in 0..j {
            let mut count = 0;
            for (aik, ajk) in a[i].iter().zip(a[j].iter()) {
                count += (*aik == *ajk) as u16;
            }
            ans += (count % 2 == 1) as u32;
        }
    }
    println!("{}", ans);
}

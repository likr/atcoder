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
        x: Chars,
    }
    if x.iter().collect::<HashSet<_>>().len() == 1
        || (1..4).all(|i| {
            let c = (x[i - 1] as u8 - '0' as u8) as usize;
            let d = (x[i] as u8 - '0' as u8) as usize;
            (d + 10 - c) % 10 == 1
        })
    {
        println!("Weak");
    } else {
        println!("Strong");
    }
}

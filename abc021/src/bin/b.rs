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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        k: usize,
        p: [usize; k],
    }
    let mut s = p.iter().map(|&pi| pi).collect::<HashSet<usize>>();
    s.insert(a);
    s.insert(b);
    if s.len() == k + 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}

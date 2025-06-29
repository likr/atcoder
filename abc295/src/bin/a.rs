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
        w: [String; n],
    }
    let t = vec![
        "and".to_string(),
        "not".to_string(),
        "that".to_string(),
        "the".to_string(),
        "you".to_string(),
    ];
    for wi in w.iter() {
        for tj in t.iter() {
            if wi == tj {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

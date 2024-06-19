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
        a: [usize],
        b: [usize],
        c: [usize],
        x: [usize],
    }
    let mut s = HashSet::new();
    for &ai in a.iter() {
        for &bi in b.iter() {
            for &ci in c.iter() {
                s.insert(ai + bi + ci);
            }
        }
    }
    for &xi in x.iter() {
        if s.contains(&xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

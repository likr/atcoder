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
        s: [String; n],
    }
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for i in 0..n {
        if s[i].starts_with('!') {
            set2.insert(s[i].clone());
        } else {
            set1.insert(s[i].clone());
        }
    }
    for i in 0..n {
        if !s[i].starts_with('!') {
            if set2.contains(&format!("!{}", s[i])) {
                println!("{}", s[i]);
                return;
            }
        }
    }
    println!("satisfiable");
}

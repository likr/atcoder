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
        mut s: Chars,
        q: usize,
        cd: [(char, char); q],
    }
    let mut chars = HashMap::new();
    for x in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut y = x;
        for i in 0..q {
            let (ci, di) = cd[i];
            if ci == y {
                y = di;
            }
        }
        chars.insert(x, y);
    }
    debug!(chars);
    for i in 0..n {
        s[i] = chars[&s[i]];
    }
    println!("{}", s.iter().collect::<String>());
}

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
        n: usize,
        mut s: [Chars; n],
    }
    let mut char_index = vec![0; 26];
    for i in 0..26 {
        char_index[x[i] as usize - 'a' as usize] = i;
    }
    s.sort_by_key(|si| {
        si.iter()
            .map(|&c| char_index[c as usize - 'a' as usize])
            .collect::<Vec<_>>()
    });
    for i in 0..n {
        println!("{}", s[i].iter().collect::<String>());
    }
}

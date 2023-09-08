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
        n: Chars,
    }
    let mut result = 0;
    for x in 0..1 << n.len() {
        let mut a = vec![];
        let mut b = vec![];
        for i in 0..n.len() {
            if x & 1 << i == 0 {
                a.push(n[i]);
            } else {
                b.push(n[i]);
            }
        }
        a.sort();
        b.sort();
        a.reverse();
        b.reverse();
        if a.len() > 0 && b.len() > 0 && a[0] != '0' && b[0] != '0' {
            let a = a.into_iter().collect::<String>().parse::<usize>().unwrap();
            let b = b.into_iter().collect::<String>().parse::<usize>().unwrap();
            result = max(result, a * b);
        }
    }
    println!("{}", result);
}

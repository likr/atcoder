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
        mut n: usize,
    }
    let mut b = vec![];
    while n > 0 {
        b.push(n % 2);
        n /= 2;
    }
    let mut ans = vec![];
    while let Some(bi) = b.pop() {
        if bi == 1 {
            ans.push('A');
        }
        ans.push('B');
    }
    ans.pop();
    let mut check = 0;
    for &c in ans.iter() {
        if c == 'A' {
            check += 1;
        } else {
            check *= 2;
        }
    }
    debug!(check);
    println!("{}", ans.iter().collect::<String>());
}

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

fn f(l: usize, r: usize, ans: &mut Vec<(usize, usize)>) {
    if l == r {
        return;
    }
    let mut k = vec![];
    for i in 0.. {
        if 1 << i > r - l {
            break;
        }
        k.push(1 << i);
    }
    while let Some(ki) = k.pop() {
        let j = (l + ki - 1) / ki;
        if l <= ki * j && ki * (j + 1) <= r {
            f(l, ki * j, ans);
            ans.push((ki * j, ki * (j + 1)));
            f(ki * (j + 1), r, ans);
            break;
        }
    }
}

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let mut ans = vec![];
    f(l, r, &mut ans);
    println!("{}", ans.len());
    for &(li, ri) in ans.iter() {
        println!("{} {}", li, ri);
    }
}

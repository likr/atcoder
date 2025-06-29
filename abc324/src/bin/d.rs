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

fn count_digit(x: usize) -> [usize; 10] {
    let mut count = [0; 10];
    let mut x = x;
    while x > 0 {
        count[x % 10] += 1;
        x /= 10;
    }
    count
}

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut s_count = [0; 10];
    for i in 0..n {
        s_count[s[i] as usize - '0' as usize] += 1;
    }
    let mut ans = 0;
    loop {
        for i in 0.. {
            if i * i > 10000000000000 {
                break;
            }
            if count_digit(i * i) == s_count {
                ans += 1;
            }
        }
        if s_count[0] == 0 {
            break;
        }
        s_count[0] -= 1;
    }
    println!("{}", ans);
}

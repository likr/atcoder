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
        mut t: Chars,
    }
    s.reverse();
    t.reverse();

    let mut s_count = HashMap::new();
    let mut t_count = HashMap::new();
    for i in 0..n {
        *s_count.entry(s[i]).or_insert(0) += 1;
        *t_count.entry(t[i]).or_insert(0) += 1;
    }
    if s_count != t_count {
        println!("-1");
        return;
    }

    let mut result = 0;
    let mut offset = 0;
    for i in 0..n {
        while offset < n && s[i] != t[offset] {
            result += 1;
            offset += 1;
        }
        if offset >= n {
            break;
        }
        offset += 1;
    }
    println!("{}", result);
}

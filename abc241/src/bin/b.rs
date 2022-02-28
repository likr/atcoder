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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
    }
    for j in 0..m {
        if count.contains_key(&b[j]) && count[&b[j]] > 0 {
            *count.entry(b[j]).or_insert(0) -= 1;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

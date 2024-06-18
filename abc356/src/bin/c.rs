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
        k: usize,
        ar: [([Usize1], char); m],
    }
    let mut ans = 0;
    for x in 0..1 << n {
        if (0..m).all(|j| {
            let mut count = 0;
            for &ai in ar[j].0.iter() {
                if x & 1 << ai > 0 {
                    count += 1;
                }
            }
            (ar[j].1 == 'o' && count >= k) || (ar[j].1 == 'x' && count < k)
        }) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

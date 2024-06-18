use ac_library::*;
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
        q: usize,
        abc: [(Usize1, Usize1, usize); n - 1 + q],
    }
    let m = 10;
    let mut dsu = vec![];
    for _ in 0..m {
        dsu.push(Dsu::new(n));
    }
    let mut ans = m * (n - 1);
    for i in 0..n - 1 + q {
        let (ai, bi, ci) = abc[i];
        for j in ci..m {
            if !dsu[j - 1].same(ai, bi) {
                dsu[j - 1].merge(ai, bi);
                ans -= 1;
            }
        }
        if i >= n - 1 {
            println!("{}", ans)
        }
    }
}

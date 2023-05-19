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
        _n: usize,
        q: usize,
    }
    let mut f = HashSet::new();
    for _ in 0..q {
        input! {
            t: usize,
            a: Usize1,
            b: Usize1,
        }
        if t == 1 {
            f.insert((a, b));
        } else if t == 2 {
            f.remove(&(a, b));
        } else {
            if f.contains(&(a, b)) && f.contains(&(b, a)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

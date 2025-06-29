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
        xy: [(i64, i64); 4],
    }
    for i in 0..4 {
        let x1 = xy[(i + 1) % 4].0 - xy[i].0;
        let y1 = xy[(i + 1) % 4].1 - xy[i].1;
        let x2 = xy[(i + 2) % 4].0 - xy[(i + 1) % 4].0;
        let y2 = xy[(i + 2) % 4].1 - xy[(i + 1) % 4].1;
        if x1 * y2 - x2 * y1 < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

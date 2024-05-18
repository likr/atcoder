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
        mut a: i64,
        mut b: i64,
        mut c: i64,
        mut d: i64,
    }
    a += 1000000000;
    b += 1000000000;
    c += 1000000000;
    d += 1000000000;
    let w4_left = (a + 3) / 4 * 4;
    let w4_right = c / 4 * 4;
    let h2_top = d / 2 * 2;
    let h2_bottom = (b + 1) / 2 * 2;
    debug!(w4_left, w4_right);
    debug!(h2_top, h2_bottom);
    let mut ans = 0;
    ans += 8 * ((w4_right - w4_left) / 4) * ((h2_top - h2_bottom) / 2);
    debug!(ans);
    let acc_right = [0, 3, 6, 7];
    ans += acc_right[(c - w4_right) as usize] * ((h2_top - h2_bottom) / 2);
    debug!(ans);
    let acc_left = [0, 1, 2, 5];
    ans += acc_left[(w4_left - a) as usize] * ((h2_top - h2_bottom) / 2);
    debug!(ans);
    if d > h2_top {
        ans += 4 * ((w4_right - w4_left) / 4);
        debug!(ans);
        let acc_right = [0, 2, 3, 3];
        ans += acc_right[(c - w4_right) as usize];
        debug!(ans);
        let acc_left = [0, 1, 1, 2];
        ans += acc_left[(w4_left - a) as usize];
        debug!(ans);
    }
    if b < h2_bottom {
        ans += 4 * ((w4_right - w4_left) / 4);
        debug!(ans);
        let acc_right = [0, 1, 3, 4];
        ans += acc_right[(c - w4_right) as usize];
        debug!(ans);
        let acc_left = [0, 0, 1, 3];
        ans += acc_left[(w4_left - a) as usize];
        debug!(ans);
    }
    println!("{}", ans)
}

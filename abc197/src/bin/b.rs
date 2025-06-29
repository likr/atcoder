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
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
    }
    let mut ans = 1;
    for (dx, dy) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
        let mut x_cur = x;
        let mut y_cur = y;
        while x_cur.wrapping_add(dx) < h && y_cur.wrapping_add(dy) < w {
            x_cur = x_cur.wrapping_add(dx);
            y_cur = y_cur.wrapping_add(dy);
            if s[x_cur][y_cur] == '#' {
                break;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}

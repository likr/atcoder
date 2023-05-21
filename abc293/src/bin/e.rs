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
        a: usize,
        mut x: usize,
        m: usize,
    }
    let mut ax = [[1, 0], [0, 1]];
    let mut t = [[a, 1], [0, 1]];
    while x > 0 {
        if x % 2 == 1 {
            ax = [
                [
                    (ax[0][0] * t[0][0] % m + ax[0][1] * t[1][0] % m) % m,
                    (ax[0][0] * t[0][1] % m + ax[0][1] * t[1][1] % m) % m,
                ],
                [
                    (ax[1][0] * t[0][0] % m + ax[1][1] * t[1][0] % m) % m,
                    (ax[1][0] * t[0][1] % m + ax[1][1] * t[1][1] % m) % m,
                ],
            ];
        }
        t = [
            [
                (t[0][0] * t[0][0] % m + t[0][1] * t[1][0] % m) % m,
                (t[0][0] * t[0][1] % m + t[0][1] * t[1][1] % m) % m,
            ],
            [
                (t[1][0] * t[0][0] % m + t[1][1] * t[1][0] % m) % m,
                (t[1][0] * t[0][1] % m + t[1][1] * t[1][1] % m) % m,
            ],
        ];
        x /= 2;
    }
    println!("{}", ax[0][1]);
}

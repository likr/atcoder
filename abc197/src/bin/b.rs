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
    let mut result = 1usize;
    for x2 in (0..x).rev() {
        if s[x2][y] == '#' {
            break;
        }
        result += 1;
    }
    for x2 in x + 1..h {
        if s[x2][y] == '#' {
            break;
        }
        result += 1;
    }
    for y2 in (0..y).rev() {
        if s[x][y2] == '#' {
            break;
        }
        result += 1;
    }
    for y2 in y + 1..w {
        if s[x][y2] == '#' {
            break;
        }
        result += 1;
    }
    println!("{}", result);
}

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
    }
    let m = 2usize.pow(n as u32);
    input! {
        a: [usize; m],
    }
    let mut win = vec![];
    win.push((0..m).collect::<Vec<_>>());
    while win[win.len() - 1].len() != 2 {
        let mut win_next = vec![];
        for i in (0..win[win.len() - 1].len()).step_by(2) {
            if a[win[win.len() - 1][i]] <  a[win[win.len() - 1][i + 1]] {
                win_next.push(win[win.len() - 1][i + 1]);
            } else {
                win_next.push(win[win.len() - 1][i]);
            }
        }
        win.push(win_next);
    }
    if a[win[win.len() - 1][0]] <  a[win[win.len() - 1][1]] {
        println!("{}", win[win.len() - 1][0] + 1);
    } else {
        println!("{}", win[win.len() - 1][1] + 1);
    }
}

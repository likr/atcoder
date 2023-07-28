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
        k: usize,
        q: usize,
        a: [Usize1; k],
        l: [usize; q],
    }
    let mut board = vec![0; n];
    for (i, &ai) in a.iter().enumerate() {
        board[ai] = i + 1;
    }
    for &li in l.iter() {
        let k = board.iter().position(|&x| x == li).unwrap();
        if k + 1 < n && board[k + 1] == 0 {
            board[k + 1] = board[k];
            board[k] = 0;
        }
    }
    println!(
        "{}",
        (0..n)
            .filter(|&i| board[i] > 0)
            .map(|i| format!("{}", i + 1))
            .collect::<Vec<_>>()
            .join(" ")
    );
}

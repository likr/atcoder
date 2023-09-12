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
        s: [Chars; h],
    }
    let patterns = vec![
        [['.', '.'], ['.', '#']],
        [['.', '.'], ['#', '.']],
        [['.', '#'], ['.', '.']],
        [['#', '.'], ['.', '.']],
        [['#', '#'], ['#', '.']],
        [['#', '#'], ['.', '#']],
        [['#', '.'], ['#', '#']],
        [['.', '#'], ['#', '#']],
    ];
    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            for k in 0..patterns.len() {
                let mut ok = true;
                for di in 0..2 {
                    for dj in 0..2 {
                        if s[i + di][j + dj] != patterns[k][di][dj] {
                            ok = false;
                        }
                    }
                }
                if ok {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

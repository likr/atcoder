use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        h: usize,
        w: usize,
        ab: [(usize, usize); n],
    }
    let mut indices = (0..n).collect::<Vec<_>>();
    loop {
        for x in 0..1 << n {
            let mut board = vec![vec![false; w]; h];
            let mut k = 0;
            let mut count = 0;
            'outer: for i in 0..h {
                for j in 0..w {
                    if board[i][j] {
                        continue;
                    }
                    while k < n {
                        let (ai, bi) = ab[indices[k]];
                        let (hi, wi) = if x & 1 << k > 0 { (ai, bi) } else { (bi, ai) };
                        k += 1;
                        if i + hi <= h && j + wi <= w {
                            let mut f = false;
                            for x in i..i + hi {
                                for y in j..j + wi {
                                    if board[x][y] {
                                        f = true;
                                    }
                                }
                            }
                            if f {
                                continue;
                            }
                            for x in i..i + hi {
                                for y in j..j + wi {
                                    board[x][y] = true;
                                    count += 1;
                                }
                            }
                            break;
                        }
                    }
                    if k == n {
                        break 'outer;
                    }
                }
            }
            if count == h * w {
                println!("Yes");
                return;
            }
        }
        if !indices.next_permutation() {
            break;
        }
    }
    println!("No");
}

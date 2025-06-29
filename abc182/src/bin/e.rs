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
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
        cd: [(Usize1, Usize1); m],
    }
    let mut grid = vec![vec!['.'; w]; h];
    for &(ci, di) in cd.iter() {
        grid[ci][di] = 'x';
    }
    for &(ai, bi) in ab.iter() {
        grid[ai][bi] = 'o';
        for &(dx, dy) in &[(!0, 0), (1, 0), (0, !0), (0, 1)] {
            let mut y = ai;
            let mut x = bi;
            while y.wrapping_add(dy) < h && x.wrapping_add(dx) < w {
                y = y.wrapping_add(dy);
                x = x.wrapping_add(dx);
                if grid[y][x] == 'x' {
                    break;
                }
                grid[y][x] = 'o'
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'o' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

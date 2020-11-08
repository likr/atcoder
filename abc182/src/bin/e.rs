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
    let mut s = vec![vec!['.'; w]; h];
    for &(ci, di) in &cd {
        s[ci][di] = '#';
    }
    let ab = ab.into_iter().collect::<HashSet<_>>();
    for i in 0..h {
        let mut on = false;
        for j in 0..w {
            if s[i][j] == '#' {
                on = false;
            } else if ab.contains(&(i, j)) {
                on = true;
            }
            if s[i][j] == '.' && on {
                s[i][j] = 'o';
            }
        }
        let mut on = false;
        for j in (0..w).rev() {
            if s[i][j] == '#' {
                on = false;
            } else if ab.contains(&(i, j)) {
                on = true;
            }
            if s[i][j] == '.' && on {
                s[i][j] = 'o';
            }
        }
    }
    for j in 0..w {
        let mut on = false;
        for i in 0..h {
            if s[i][j] == '#' {
                on = false;
            } else if ab.contains(&(i, j)) {
                on = true;
            }
            if s[i][j] == '.' && on {
                s[i][j] = 'o';
            }
        }
        let mut on = false;
        for i in (0..h).rev() {
            if s[i][j] == '#' {
                on = false;
            } else if ab.contains(&(i, j)) {
                on = true;
            }
            if s[i][j] == '.' && on {
                s[i][j] = 'o';
            }
        }
    }
    let mut result = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                result += 1;
            }
        }
    }
    println!("{}", result);
}

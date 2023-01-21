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

fn rec(
    c: &Vec<Vec<char>>,
    i0: usize,
    j0: usize,
    i: usize,
    j: usize,
    visited: &mut Vec<Vec<bool>>,
    depth: usize,
) -> Option<usize> {
    let mut result = None;
    for &(dr, dc) in [(1, 0), (1, 2), (0, 1), (2, 1)].iter() {
        let i2 = i + dr - 1;
        let j2 = j + dc - 1;
        if i0 == i2 && j0 == j2 && depth > 2 {
            result = max(result, Some(depth + 1));
        } else if c[i2][j2] == '.' && !visited[i2][j2] {
            visited[i2][j2] = true;
            result = max(result, rec(c, i0, j0, i2, j2, visited, depth + 1));
            visited[i2][j2] = false;
        }
    }
    result
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c0: [Chars; h],
    }
    let mut c = vec![vec!['#'; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            c[i + 1][j + 1] = c0[i][j];
        }
    }

    let mut result = None;
    for i in 1..=h {
        for j in 1..=w {
            if c[i][j] == '.' {
                let mut visited = vec![vec![false; w + 2]; h + 2];
                visited[i][j] = true;
                result = max(result, rec(&c, i, j, i, j, &mut visited, 0))
            }
        }
    }
    if let Some(result) = result {
        println!("{}", result);
    } else {
        println!("-1");
    }
}

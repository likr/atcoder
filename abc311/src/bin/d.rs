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
        m: usize,
        s: [Chars; n],
    }
    let mut ans = vec![vec![false; m]; n];
    ans[1][1] = true;
    let mut visited = vec![vec![false; m]; n];
    visited[1][1] = true;
    let mut queue = VecDeque::new();
    queue.push_back((1usize, 1usize));
    while let Some((i, j)) = queue.pop_front() {
        for &(di, dj) in &[(0, 1), (0, !0), (1, 0), (!0, 0)] {
            let mut i2 = i;
            let mut j2 = j;
            while s[i2.wrapping_add(di)][j2.wrapping_add(dj)] == '.' {
                ans[i2][j2] = true;
                i2 = i2.wrapping_add(di);
                j2 = j2.wrapping_add(dj);
            }
            if !visited[i2][j2] {
                visited[i2][j2] = true;
                queue.push_back((i2, j2))
            }
        }
    }
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if ans[i][j] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

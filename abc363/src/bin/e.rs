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
        y: usize,
        a: [[usize; w]; h],
    }
    let mut ss = BinaryHeap::new();
    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                ss.push((Reverse(a[i][j]), i, j));
            }
        }
    }
    let mut t = vec![vec![INF; w]; h];
    let mut visited = vec![vec![false; w]; h];
    while let Some((Reverse(t0), si, sj)) = ss.pop() {
        if visited[si][sj] {
            continue;
        }
        debug!(t0, si, sj);
        let mut queue = VecDeque::new();
        queue.push_back((si, sj));
        t[si][sj] = t0;
        visited[si][sj] = true;
        while let Some((i, j)) = queue.pop_front() {
            for &(di, dj) in [(0, !0), (0, 1), (!0, 0), (1, 0)].iter() {
                let i2 = i.wrapping_add(di);
                let j2 = j.wrapping_add(dj);
                if i2 < h && j2 < w && !visited[i2][j2] {
                    if a[i2][j2] <= t0 {
                        queue.push_back((i2, j2));
                        t[i2][j2] = t0;
                        visited[i2][j2] = true;
                    } else {
                        ss.push((Reverse(a[i2][j2]), i2, j2));
                    }
                }
            }
        }
    }
    let mut count = vec![0; y + 1];
    for i in 0..h {
        for j in 0..w {
            count[min(y, t[i][j] - 1)] += 1;
        }
    }
    debug!(count);
    for i in (0..y).rev() {
        count[i] += count[i + 1];
    }
    for i in 1..=y {
        println!("{}", count[i]);
    }
}

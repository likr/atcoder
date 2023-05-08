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
        k: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        c0: [Chars; h],
    }
    let mut c = vec![vec!['@'; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            c[i][j] = c0[i - 1][j - 1];
        }
    }
    let mut distance = vec![vec![vec![(INF, INF); 4]; w + 2]; h + 2];
    let mut heap = BinaryHeap::new();
    let ds = [(0, 1), (2, 1), (1, 0), (1, 2)];
    for i in 0..4 {
        distance[x1][y1][i] = (0, 0);
        heap.push((Reverse(0), Reverse(0), i, x1, y1));
    }
    while let Some((Reverse(d), Reverse(e), f, x0, y0)) = heap.pop() {
        if distance[x0][y0][f] < (d, e) {
            continue;
        }
        {
            let x = x0 + ds[f].0 - 1;
            let y = y0 + ds[f].1 - 1;
            if c[x][y] == '.' && distance[x][y][f] > (d + (e + 1) / k, (e + 1) % k) {
                let (dd, de) = (d + (e + 1) / k, (e + 1) % k);
                distance[x][y][f] = (dd, de);
                heap.push((Reverse(dd), Reverse(de), f, x, y));
            }
        }
        for i in 0..4 {
            if i / 2 != f / 2 && distance[x0][y0][i] > (d + (e + k - 1) / k, 0) {
                let (dd, de) = (d + (e + k - 1) / k, 0);
                distance[x0][y0][i] = (dd, de);
                heap.push((Reverse(dd), Reverse(de), i, x0, y0));
            }
        }
    }
    let result = (0..4)
        .map(|i| distance[x2][y2][i].0 + (distance[x2][y2][i].1 + k - 1) / k)
        .min()
        .unwrap();
    if result >= INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}

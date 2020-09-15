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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
    #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn solve(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    index: &Vec<(usize, usize)>,
    x: &Vec<usize>,
    y: &Vec<usize>,
    cache: &mut HashMap<(usize, usize, usize, usize), usize>,
) -> usize {
    if let Some(&s) = cache.get(&(x1, y1, x2, y2)) {
        s
    } else {
        let mut s = 0;
        for &(xi, yi) in index {
            if x1 < xi && xi < x2 && y1 < yi && yi < y2 {
                let mut t = x[x2] - x[x1] + y[y2] - y[y1] - 3;
                t += solve(x1, y1, xi, yi, index, x, y, cache);
                t += solve(x1, yi, xi, y2, index, x, y, cache);
                t += solve(xi, y1, x2, yi, index, x, y, cache);
                t += solve(xi, yi, x2, y2, index, x, y, cache);
                s = max(s, t);
            }
        }
        cache.insert((x1, y1, x2, y2), s);
        s
    }
}

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        mut xy: [(usize, usize); n],
    }

    let index = xy
        .iter()
        .map(|&(xi, yi)| {
            (
                xy.iter().filter(|&&(xj, _)| xj <= xi).count(),
                xy.iter().filter(|&&(_, yj)| yj <= yi).count(),
            )
        })
        .collect::<Vec<(usize, usize)>>();
    debug!(index);

    let mut x = vec![0; n + 2];
    x[0] = 0;
    x[n + 1] = w + 1;
    xy.sort_by_key(|&(xi, _)| xi);
    for i in 0..n {
        x[i + 1] = xy[i].0;
    }

    let mut y = vec![0; n + 2];
    y[0] = 0;
    y[n + 1] = h + 1;
    xy.sort_by_key(|&(_, yi)| yi);
    for i in 0..n {
        y[i + 1] = xy[i].1;
    }

    debug!(x);
    debug!(y);

    let mut cache = HashMap::new();
    let result = solve(0, 0, n + 1, n + 1, &index, &x, &y, &mut cache);
    debug!(cache);
    println!("{}", result);
}

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

fn solve(
    i: usize,
    y: i64,
    cache: &mut HashMap<(usize, i64), usize>,
    a: &Vec<i64>,
    k: &Vec<i64>,
    max_y: &Vec<i64>,
) -> usize {
    debug!(i, y);
    if let Some(&p) = cache.get(&(i, y)) {
        p
    } else {
        let result = if i == 0 {
            if y.abs() <= max_y[i] {
                1
            } else {
                0
            }
        } else if y.abs() > max_y[i] {
            0
        } else {
            let y1 = y % a[i];
            let mut s = 0;
            if (y - y1).abs() / a[i] <= k[i] {
                s += solve(i - 1, y1, cache, a, k, max_y);
            }
            let y2 = y1 + if y < 0 { a[i] } else { -a[i] };
            if (y - y2).abs() / a[i] <= k[i] {
                s += solve(i - 1, y2, cache, a, k, max_y);
            }
            s
        };
        cache.insert((i, y), result);
        result
    }
}

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }
    let mut k = vec![0; n];
    for i in 1..n {
        k[i - 1] = a[i] / a[i - 1] - 1;
    }
    k[n - 1] = (x + a[n - 1] - 1) / a[n - 1];
    debug!(k);

    let mut y0 = vec![];
    let mut x0 = x;
    for i in (0..n).rev() {
        y0.push(x0 / a[i]);
        x0 %= a[i];
    }
    y0.reverse();
    debug!(y0);

    let mut max_y = vec![0; n];
    max_y[0] = k[0];
    for i in 1..n {
        max_y[i] = max_y[i - 1] + a[i] * k[i];
    }
    debug!(max_y);

    let mut cache = HashMap::new();
    println!("{}", solve(n - 1, x, &mut cache, &a, &k, &max_y));
    debug!(cache);
}

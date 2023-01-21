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

fn gcd(a: i64, b: i64) -> i64 {
    if b > a {
        gcd(b, a)
    } else if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut xy: [(i64, i64); n],
    }
    if k == 1 {
        println!("Infinity");
        return;
    }
    let mut result = HashSet::new();
    for i in 0..n {
        let (x0, y0) = xy[i];
        for j in 0..n {
            xy[j].0 -= x0;
            xy[j].1 -= y0;
        }
        let mut count = HashMap::new();
        for j in 0..n {
            let (xj, yj) = xy[j];
            if xj == 0 && yj == 0 {
                continue;
            } else if xj == 0 {
                *count.entry((0, 1)).or_insert(0) += 1;
            } else if yj == 0 {
                *count.entry((1, 0)).or_insert(0) += 1;
            } else {
                let g = gcd(xj.abs(), yj.abs());
                let sign = if (xj < 0 && yj >= 0) || (xj >= 0 && yj < 0) {
                    -1
                } else {
                    1
                };
                *count
                    .entry((sign * xj.abs() / g, yj.abs() / g))
                    .or_insert(0) += 1;
            }
        }
        for &key in count.keys() {
            let v = count[&key];
            if v + 1 >= k {
                let (x, y) = key;
                if y == 0 {
                    result.insert((x, y, y0));
                } else if x == 0 {
                    result.insert((x, y, x0));
                } else {
                    result.insert((x, y, x * y0 - y * x0));
                }
            }
        }
        for j in 0..n {
            xy[j].0 += x0;
            xy[j].1 += y0;
        }
    }
    println!("{}", result.len());
}

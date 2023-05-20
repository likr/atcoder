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
        a: [usize; n],
        b: [usize; n],
        c: [usize; m],
        d: [usize; m],
    }
    let mut points = vec![];
    for i in 0..n {
        points.push((a[i], b[i], 0, i));
    }
    for i in 0..m {
        points.push((c[i], d[i], 1, i));
    }
    points.sort();
    let mut visited = BTreeSet::new();
    for &(_yi, xi, t, i) in points.iter() {
        if t == 0 {
            visited.insert((xi, i));
        } else {
            if let Some(&(xj, j)) = visited.range(..=(xi, INF)).rev().nth(0) {
                visited.remove(&(xj, j));
            }
        }
    }
    if visited.len() > 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}

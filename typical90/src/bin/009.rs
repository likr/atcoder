use ordered_float::OrderedFloat;
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
        xy: [(f64, f64); n],
    }
    let mut result = 0.;
    let mut angles = vec![];
    for i in 0..n {
        let (xi, yi) = xy[i];
        angles.clear();
        for j in 0..n {
            if i != j {
                let (xj, yj) = xy[j];
                let t = (yj - yi).atan2(xj - xi);
                if t >= 0. {
                    angles.push(t);
                    angles.push(t + 2. * PI);
                } else {
                    angles.push(t + 2. * PI);
                    angles.push(t + 4. * PI);
                }
            }
        }
        angles.sort_by_key(|&v| OrderedFloat(v));
        let mut k = 0;
        for j in 0..n - 1 {
            while angles[k + 1] - angles[j] <= PI {
                k += 1;
            }
            let t = angles[k] - angles[j];
            if t > result {
                result = t;
            }
        }
    }
    println!("{}", result.to_degrees());
}

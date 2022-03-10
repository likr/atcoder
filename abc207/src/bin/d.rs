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
        mut cd_orig: [(i64, i64); n],
        mut ab: [(i64, i64); n],
    }
    let a_min = ab.iter().map(|&(ai, _)| ai).min().unwrap();
    let b_min = ab.iter().map(|&(_, bi)| bi).min().unwrap();
    for i in 0..n {
        ab[i].0 -= a_min;
        ab[i].1 -= b_min;
    }
    let mut squares = HashMap::new();
    for i in 1.. {
        if i * i > 1000000 {
            break;
        }
        squares.insert(i * i, i);
    }
    if n == 2 {
        let dx1 = ab[0].0 - ab[1].0;
        let dy1 = ab[0].1 - ab[1].1;
        let dx2 = cd_orig[0].0 - cd_orig[1].0;
        let dy2 = cd_orig[0].1 - cd_orig[1].1;
        if dx1 * dx1 + dy1 * dy1 == dx2 * dx2 + dy2 * dy2 {
            println!("Yes");
            return;
        }
    }
    for i in 0..n {
        let (ci, di) = cd_orig[i];
        let cd = cd_orig
            .iter()
            .map(|&(cj, dj)| (cj - ci, dj - di))
            .collect::<Vec<_>>();
        for a in -50..=50 {
            for b in -50..=50 {
                if let Some(&c) = squares.get(&(a * a + b * b)) {
                    if !cd
                        .iter()
                        .all(|&(ci, di)| (ci * a - di * b) % c == 0 && (ci * b + di * a) % c == 0)
                    {
                        continue;
                    }
                    if a != 0 && b != 0 {
                        debug!(a, b, c);
                    }
                    let mut tmp = cd
                        .iter()
                        .map(|&(ci, di)| ((ci * a - di * b) / c, (ci * b + di * a) / c))
                        .collect::<Vec<_>>();
                    let x_min = tmp.iter().map(|&(x, _)| x).min().unwrap();
                    let y_min = tmp.iter().map(|&(_, y)| y).min().unwrap();
                    for i in 0..n {
                        tmp[i].0 -= x_min;
                        tmp[i].1 -= y_min;
                    }
                    if (0..n).all(|i| (0..n).any(|j| ab[i] == tmp[j])) {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}

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
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    if sx == tx && sy == ty {
        println!("Yes");
        return;
    }
    let mut img = vec![((tx, ty, tx, ty))];
    for i in 1..=100001 {
        let (x0, y0, x1, y1) = img[i - 1];
        let x_cand = vec![2 * a - x0, 2 * b - x0, 2 * a - x1, 2 * b - x1];
        let y_cand = vec![2 * c - y0, 2 * d - y0, 2 * c - y1, 2 * d - y1];
        img.push((
            *x_cand.iter().min().unwrap(),
            *y_cand.iter().min().unwrap(),
            *x_cand.iter().max().unwrap(),
            *y_cand.iter().max().unwrap(),
        ));
        debug!(img[i]);
        if img[i].0 <= sx && sx <= img[i].2 && img[i].1 <= sy && sy <= img[i].3 {
            break;
        }
    }
    let n = img.len() - 1;
    if n == 100001 {
        println!("No");
        return;
    }
    img.reverse();
    let mut x = sx;
    let mut y = sy;
    let mut ans = vec![];
    for i in 0..n {
        let (x0, y0, x1, y1) = img[i];
        let p_x = ((x - x0) * (b - a) + (x1 - x0) * a) / (x1 - x0);
        let p_y = ((y - y0) * (d - c) + (y1 - y0) * c) / (y1 - y0);
        x = p_x * 2 - x;
        y = p_y * 2 - y;
        ans.push((p_x, p_y));
    }
    debug!(x, y, tx, ty);
    println!("Yes");
    for &(xi, yi) in ans.iter() {
        println!("{} {}", xi, yi);
    }
}

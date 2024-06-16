use ac_library::*;
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
        m: usize,
        mut xy: [(Usize1, Usize1); m],
    }
    let mut min_x = vec![h; w];
    let mut min_y = vec![w; h];
    let mut row_y = vec![vec![]; h];
    for i in 0..m {
        let (xi, yi) = xy[i];
        min_x[yi] = min(min_x[yi], xi);
        min_y[xi] = min(min_y[xi], yi);
        row_y[xi].push(yi);
    }

    let mut ans = 0usize;
    for y in 0..min_y[0] {
        ans += min_x[y];
    }

    let mut segtree = Segtree::<Additive<usize>>::new(w);
    for y in min_y[0]..w {
        segtree.set(y, 1);
    }
    for x in 1..min_x[0] {
        ans += segtree.prod(0..min_y[x]);
        for &y in row_y[x].iter() {
            segtree.set(y, 1);
        }
    }
    println!("{}", ans);
}

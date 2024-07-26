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
        n: usize,
        mut m: usize,
        tx: [(usize, usize); n],
    }
    let mut p = vec![];
    for i in 0..n {
        let (ti, xi) = tx[i];
        if ti == 2 {
            p.push((0, xi, ti));
        } else {
            p.push((xi, 0, ti));
        }
    }
    p.sort();
    p.reverse();

    let mut count = Segtree::<Additive<usize>>::new(n);
    let mut value = Segtree::<Additive<usize>>::new(n);
    let mut x1 = vec![];
    let mut x2 = vec![];
    for i in 0..n {
        if p[i].2 == 0 {
            count.set(i, 1);
            value.set(i, p[i].0)
        } else if p[i].2 == 1 {
            x1.push((p[i].0, i));
        } else {
            x2.push(p[i].1);
        }
    }
    x1.reverse();
    x2.reverse();

    let mut ans = 0;
    loop {
        let k = min(n, count.max_right(0, |&s| s <= m));
        ans = max(ans, value.prod(0..k));
        if m == 0 || x2.is_empty() {
            break;
        }
        let k = x2.pop().unwrap();
        for _ in 0..k {
            if let Some((v, i)) = x1.pop() {
                count.set(i, 1);
                value.set(i, v);
            } else {
                break;
            }
        }
        m -= 1;
    }
    println!("{}", ans);
}

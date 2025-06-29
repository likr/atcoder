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
        s: Chars,
        t: Chars,
    }
    let mut index = vec![];
    for _ in 0..26 {
        index.push(Segtree::<Additive<usize>>::new(s.len()));
    }
    for (i, &c) in s.iter().enumerate() {
        index[c as usize - 'a' as usize].set(i, 1);
    }
    for &c in t.iter() {
        if index[c as usize - 'a' as usize].all_prod() == 0 {
            println!("0");
            return;
        }
    }
    let mut ok = 0;
    let mut ng = INF;
    while ng - ok > 1 {
        let k = (ok + ng) / 2;
        let mut offset = 0;
        let mut repeat = 0;
        for &c in t.iter() {
            let mut x = k;
            let a = index[c as usize - 'a' as usize].prod(0..offset);
            let b = index[c as usize - 'a' as usize].all_prod();
            debug!(c, a, b, x, offset, repeat);
            if x <= b - a {
                offset = index[c as usize - 'a' as usize].max_right(offset, |&s| s < x + a) + 1;
            } else {
                repeat += 1;
                x -= b - a;
                debug!(x, b);
                if x % b == 0 {
                    repeat += x / b - 1;
                    x -= b * (x / b - 1);
                    offset = index[c as usize - 'a' as usize].max_right(0, |&s| s < x) + 1;
                } else {
                    repeat += x / b;
                    x = x % b;
                    offset = index[c as usize - 'a' as usize].max_right(0, |&s| s < x) + 1;
                }
                debug!(offset);
            }
            if offset >= s.len() {
                offset = 0;
                repeat += 1;
            }
        }
        if offset > 0 {
            repeat += 1;
        }
        debug!(k, repeat);
        if repeat <= n {
            ok = k;
        } else {
            ng = k;
        }
    }
    println!("{}", ok);
}

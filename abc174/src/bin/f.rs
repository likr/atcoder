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
        q: usize,
        c: [Usize1; n],
        lr: [(Usize1, Usize1); q],
    }
    let mut events = BinaryHeap::new();
    for i in 0..q {
        events.push((Reverse(lr[i].1), i));
    }

    let mut indices = vec![INF; n];
    let mut count = Segtree::<Additive<usize>>::new(n);
    let mut ans = vec![0; q];
    for t in 0..n {
        if indices[c[t]] < INF {
            count.set(indices[c[t]], 0);
        }
        indices[c[t]] = t;
        count.set(t, 1);
        while let Some(&(Reverse(ri), i)) = events.peek() {
            if ri != t {
                break;
            }
            events.pop();
            let li = lr[i].0;
            ans[i] = count.prod(li..=ri);
        }
    }
    for i in 0..q {
        println!("{}", ans[i]);
    }
}

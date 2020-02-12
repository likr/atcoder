use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut td: [(usize, usize); n],
    }
    td.sort_by_key(|&(_, di)| di);
    td.reverse();

    let mut s = 0;
    let mut heap = BinaryHeap::new();
    let mut ts = HashSet::new();
    for i in 0..k {
        let (ti, di) = td[i];
        if ts.contains(&ti) {
            heap.push(Reverse(di));
        } else {
            s += ts.len() * 2 + 1;
            ts.insert(ti);
        }
        s += di;
    }
    let mut d = vec![];
    let mut i = k;
    while let Some(Reverse(dj)) = heap.pop() {
        while i < n && ts.contains(&td[i].0) {
            i += 1;
        }
        if i < n {
            let (ti, di) = td[i];
            d.push((ts.len() * 2 + 1 + di) as i64 - dj as i64);
            ts.insert(ti);
        }
    }
    if d.len() > 0 {
        // eprintln!("{:?}", d);
        let mut dsum = vec![0; d.len()];
        dsum[0] = d[0];
        for i in 1..d.len() {
            dsum[i] = dsum[i - 1] + d[i];
        }
        // eprintln!("{:?}", dsum);
        if let Some(&d_max) = dsum.iter().max() {
            if d_max > 0 {
                s += d_max as usize;
            }
        }
    }
    println!("{}", s);
}

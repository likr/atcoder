use proconio::input;
use std::cmp::{max, min};
use superslice::*;

fn main() {
    input! {
      a: usize,
      b: usize,
      q: usize,
      mut s: [i64; a],
      mut t: [i64; b],
      x: [i64; q],
    }
    let inf = 1000000000000;
    s.push(-inf);
    s.push(inf);
    s.sort();
    t.push(-inf);
    t.push(inf);
    t.sort();
    for &xi in &x {
        let i = s.lower_bound(&xi);
        let j = t.lower_bound(&xi);
        let s0 = s[i - 1];
        let s1 = s[i];
        let t0 = t[j - 1];
        let t1 = t[j];
        let mut d = inf;
        d = min(xi - min(s0, t0), d);
        d = min(max(s1, t1) - xi, d);
        d = min(s1 - xi + s1 - t0, d);
        d = min(t1 - xi + t1 - s0, d);
        d = min(xi - s0 + t1 - s0, d);
        d = min(xi - t0 + s1 - t0, d);
        println!("{}", d);
    }
}

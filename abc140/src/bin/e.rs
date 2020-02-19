use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      p: [usize; n],
    }
    let mut indices = BTreeSet::new();
    indices.insert(0);
    indices.insert(1);
    indices.insert(n + 2);
    indices.insert(n + 3);

    let mut p = p
        .into_iter()
        .enumerate()
        .map(|(i, pi)| (pi, i + 2))
        .collect::<BinaryHeap<_>>();

    let mut s = 0;
    while let Some((pi, i)) = p.pop() {
        let mut left = indices.range(..i).rev();
        let mut right = indices.range(i..);
        let &l1 = left.next().unwrap();
        let &l2 = left.next().unwrap();
        let &r1 = right.next().unwrap();
        let &r2 = right.next().unwrap();
        // eprintln!("{}", pi);
        // eprintln!("{} {} {} {} {}", l2, l1, i, r1, r2);
        if 1 < l1 {
            s += pi * (l1 - l2) * (r1 - i);
        }
        if r1 < n + 2 {
            s += pi * (r2 - r1) * (i - l1);
        }
        // eprintln!(" {}", s);
        indices.insert(i);
    }
    println!("{}", s);
}

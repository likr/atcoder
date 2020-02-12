use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

struct SegmentTree {
    buffer: Vec<(i64, i64)>,
}

impl SegmentTree {
    fn new(n: usize) -> SegmentTree {
        let mut m = 1;
        while (m < n) {
            m *= 2;
        }
        SegmentTree {
            buffer: vec![(0, 0); 2 * m - 1],
        }
    }

    fn set(&mut self, l: usize, r: usize, v: i64) {}

    fn set_inner(&mut self, l: usize, r: usize, v: i64, k: usize) {}

    fn split(&self, l: usize, r: usize, k: usize) -> Option<(usize, usize)> {
        let (l2, r2) = self.range(k);
        if l == l2 && r = r2 {
            return None;
        }
    }

    fn len(&self) -> usize {
        self.buffer.len() / 2 + 1
    }

    fn height(&self, k: usize) -> usize {
        let mut h = 0;
        let mut k = k + 1;
        while k > 0 {
            h += 1;
            k /= 2;
        }
        h
    }

    fn range(&self, k: usize) -> usize {
        let mut m = self.len();
        for _ in 0..self.height(k) {
            m /= 2;
        }
        let n = self.len() / m;
        let l = (k + 1) % n * m;
        (l, l + m)
    }
}

fn main() {
    input! {
      n: usize,
      q: usize,
      stx: [(i64, i64, i64); n],
      d: [i64; q],
    }

    let mut events = BinaryHeap::new();
    for &(sk, tk, xk) in &stx {
        events.push(Reverse((sk - xk, false, xk)));
        events.push(Reverse((tk - xk, true, xk)));
    }

    let mut x = vec![];
    for &ti in &d {
        while let Some(&Reverse((t, remove, xk))) = events.peek() {
            if t > ti {
                break;
            }
            if remove {
                let k = x.lower_bound(&xk);
                x.remove(k);
            } else {
                let k = x.lower_bound(&xk);
                x.insert(k, xk);
            }
            events.pop();
        }
        if x.is_empty() {
            println!("-1");
        } else {
            println!("{}", x[0]);
        }
    }
}

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
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

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

    let mut x = BTreeMap::new();
    for &ti in &d {
        while let Some(&Reverse((t, remove, xk))) = events.peek() {
            if t > ti {
                break;
            }
            if remove {
                let c = x.get_mut(&xk).unwrap();
                *c -= 1;
                if *c == 0 {
                    x.remove(&xk);
                }
            } else {
                *x.entry(xk).or_insert(0) += 1;
            }
            events.pop();
        }
        if let Some((xk, _)) = x.iter().nth(0) {
            println!("{}", xk);
        } else {
            println!("-1");
        }
    }
}

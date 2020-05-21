use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut stx: [(isize, isize, isize); n],
        d: [isize; q],
    }
    stx.sort_by_key(|&(si, ti, xi)| (xi, si, ti));

    let mut events = BinaryHeap::new();
    for &(si, ti, xi) in &stx {
        let left = d.lower_bound(&(si - xi));
        let right = d.lower_bound(&(ti - xi));
        events.push(Reverse((left, xi, true)));
        events.push(Reverse((right, xi, false)));
    }

    let mut block = BTreeMap::new();
    let mut x = vec![-1; q];
    for i in 0..q {
        // eprintln!("{}", i);
        while let Some(&Reverse((k, xk, insert))) = events.peek() {
            if k > i {
                break;
            }
            // eprintln!("{} {} {}", k, xk, insert);
            if insert {
                *block.entry(xk).or_insert(0) += 1;
                if block[&xk] == 0 {
                    block.remove(&xk);
                }
            } else {
                *block.entry(xk).or_insert(0) -= 1;
                if block[&xk] == 0 {
                    block.remove(&xk);
                }
            }
            events.pop();
        }
        if !block.is_empty() {
            x[i] = *block.range(0..).next().unwrap().0;
        }
    }
    for i in 0..q {
        println!("{}", x[i]);
    }
}

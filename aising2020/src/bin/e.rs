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

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            klr: [(Usize1, isize, isize); n],
        }
        let mut left = vec![];
        let mut right = vec![];
        let mut s = 0;
        for i in 0..n {
            let (_, li, ri) = klr[i];
            if li > ri {
                left.push(i);
            } else if li < ri {
                right.push(i);
            } else {
                s += li;
            }
        }

        left.sort_by_key(|&i| klr[i].0);
        left.reverse();
        let mut left_heap = BinaryHeap::new();
        for t in 0..n {
            while let Some(i) = left.pop() {
                let (ki, li, ri) = klr[i];
                if ki <= t {
                    left_heap.push((Reverse(li - ri), i));
                } else {
                    left.push(i);
                    break;
                }
            }
            while left_heap.len() > t + 1 {
                let (_, i) = left_heap.pop().unwrap();
                s += klr[i].2;
            }
        }
        while let Some((_, i)) = left_heap.pop() {
            s += klr[i].1;
        }

        right.sort_by_key(|&i| klr[i].0);
        let mut right_heap = BinaryHeap::new();
        for t in (0..n).rev() {
            while let Some(i) = right.pop() {
                let (ki, li, ri) = klr[i];
                if ki >= t {
                    right_heap.push((Reverse(ri - li), i));
                } else {
                    right.push(i);
                    break;
                }
            }
            while right_heap.len() > n - t - 1 {
                let (_, i) = right_heap.pop().unwrap();
                s += klr[i].1;
            }
        }
        while let Some((_, i)) = right_heap.pop() {
            s += klr[i].2;
        }

        println!("{}", s);
    }
}

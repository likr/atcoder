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
      p: [usize; n],
    }
    let mut left = vec![None; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        while let Some(&Reverse((q, _))) = heap.peek() {
            if q > p[i] {
                break;
            }
            heap.pop();
        }
        if let Some(&Reverse((_, j))) = heap.peek() {
            left[i] = Some(j);
        }
        heap.push(Reverse((p[i], i)));
    }
    let mut right = vec![None; n];
    let mut heap = BinaryHeap::new();
    for i in (0..n).rev() {
        while let Some(&Reverse((q, _))) = heap.peek() {
            if q > p[i] {
                break;
            }
            heap.pop();
        }
        if let Some(&Reverse((_, j))) = heap.peek() {
            right[i] = Some(j);
        }
        heap.push(Reverse((p[i], i)));
    }
    eprintln!("{:?}", left);
    eprintln!("{:?}", right);
    let mut left_count = vec![0; n];
    let mut right_count = vec![0; n];
    for i in 0..n {
        left_count[i] = if let Some(j) = left[i] { i - j - 1 } else { i };
        right_count[i] = if let Some(j) = right[i] {
            j - i - 1
        } else {
            n - i - 1
        };
    }
    eprintln!("{:?}", left_count);
    eprintln!("{:?}", right_count);
    let mut result = 0;
    for i in 0..n {
        if let Some(j) = left[i] {
            result += p[i] * (left_count[j] + 1) * (right_count[i] + 1);
        }
        if let Some(j) = right[i] {
            result += p[i] * (left_count[i] + 1) * (right_count[j] + 1);
        }
    }
    println!("{}", result);
}

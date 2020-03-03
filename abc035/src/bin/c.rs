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
        n: usize,
        q: usize,
        lr: [(Usize1, Usize1); q],
    }
    let mut heap = BinaryHeap::new();
    for &(li, ri) in &lr {
        heap.push(Reverse((li, false)));
        heap.push(Reverse((ri + 1, false)));
    }
    let mut c = 0;
    for i in 0..n {
        while let Some(&Reverse((k, remove))) = heap.peek() {
            if k == i {
                if remove {
                    c -= 1;
                } else {
                    c += 1;
                }
                heap.pop();
            } else {
                break;
            }
        }
        // eprintln!("{} {}", i, c);
        print!("{}", if c % 2 == 0 { '0' } else { '1' });
    }
    println!("");
}

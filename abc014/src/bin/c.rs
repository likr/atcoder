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
        ab: [(usize, usize); n],
    }
    let mut events = BinaryHeap::new();
    for &(ai, bi) in &ab {
        events.push(Reverse((ai, false)));
        events.push(Reverse((bi + 1, true)));
    }
    let mut result = 0;
    let mut c = 0;
    for i in 0..=1000000 {
        while let Some(&Reverse((j, remove))) = events.peek() {
            if i != j {
                break;
            }
            if remove {
                c -= 1;
            } else {
                c += 1;
            }
            events.pop();
        }
        result = max(result, c);
    }
    println!("{}", result);
}

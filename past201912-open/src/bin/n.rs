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
        w: isize,
        c: isize,
        lrp: [(isize, isize, usize); n],
    }
    let mut events = BinaryHeap::new();
    for &(li, ri, pi) in &lrp {
        events.push(Reverse((li - c, true, pi)));
        events.push(Reverse((ri, false, pi)));
    }

    let mut result = INF;
    let mut cost = 0;
    while let Some(&Reverse((t, _, _))) = events.peek() {
        while let Some(&Reverse((s, plus, p))) = events.peek() {
            if s != t || plus {
                break;
            }
            cost -= p;
            events.pop();
        }
        // eprintln!("{} {}", t, cost);
        if 0 <= t && t + c <= w {
            result = min(result, cost);
        }
        while let Some(&Reverse((s, _, p))) = events.peek() {
            if s != t {
                break;
            }
            cost += p;
            events.pop();
        }
    }
    println!("{}", result);
}

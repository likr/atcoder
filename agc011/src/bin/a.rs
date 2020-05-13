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
        c: usize,
        k: usize,
       t: [usize; n],
    }

    let mut events = BinaryHeap::new();
    for i in 0..n {
        events.push(Reverse((t[i], false, i)));
        events.push(Reverse((t[i] + k, true, i)));
    }

    let mut result = 0;
    let mut leaved = vec![false; n];
    let mut waiting = VecDeque::new();
    while let Some(Reverse((_, remove, i))) = events.pop() {
        if remove {
            if leaved[i] {
                continue;
            }
            let mut d = 0;
            while let Some(j) = waiting.pop_front() {
                leaved[j] = true;
                d += 1;
                if d >= c {
                    break;
                }
            }
            result += 1;
        } else {
            waiting.push_back(i);
        }
    }
    println!("{}", result);
}

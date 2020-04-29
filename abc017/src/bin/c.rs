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
        m: usize,
        lrs: [(Usize1, Usize1, isize); n],
    }
    let mut events = BinaryHeap::new();
    for &(li, ri, si) in &lrs {
        events.push(Reverse((li, si, false)));
        events.push(Reverse((ri + 1, si, true)));
    }

    let mut score = vec![0isize; m];
    let mut t = 0;
    for i in 0..m {
        while let Some(&Reverse((j, s, remove))) = events.peek() {
            if i != j {
                break;
            }
            if remove {
                t -= s;
            } else {
                t += s;
            }
            events.pop();
        }
        score[i] = t;
    }

    println!(
        "{}",
        lrs.iter().map(|&(_, _, s)| s).sum::<isize>() - score.iter().min().unwrap()
    );
}

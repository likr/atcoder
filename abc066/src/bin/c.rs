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
        a: [usize; n],
    }
    let mut b = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            b.push_back(a[i]);
        } else {
            b.push_front(a[i]);
        }
    }
    if n % 2 == 0 {
        while let Some(bi) = b.pop_front() {
            print!("{} ", bi);
        }
    } else {
        while let Some(bi) = b.pop_back() {
            print!("{} ", bi);
        }
    }
}

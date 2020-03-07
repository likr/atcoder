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
        s: Chars,
    }
    let mut queue = VecDeque::new();
    for &c in &s {
        if c == 'B' {
            queue.pop_back();
        } else {
            queue.push_back(c);
        }
    }
    while let Some(c) = queue.pop_front() {
        print!("{}", c);
    }
    println!("");
}

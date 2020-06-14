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
    }
    let mut a = (0..n).collect::<Vec<_>>();
    let mut events = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1,
                _y: usize,
            }
            a.swap(x, x + 1);
        } else {
            input! {
                x: Usize1,
                y: Usize1,
            }
            events.push(Reverse(x));
            events.push(Reverse(y + 1));
        }
    }
    let mut reversed = false;
    let mut result = vec![];
    for i in 0..n {
        while let Some(&Reverse(k)) = events.peek() {
            if k > i {
                break;
            }
            reversed = !reversed;
            events.pop();
        }
        if reversed {
            result.push(a[n - 1 - i]);
        } else {
            result.push(a[i]);
        }
    }
    eprintln!("{:?}", a);
    for i in 0..n {
        print!("{} ", result[i] + 1);
    }
    println!("");
}

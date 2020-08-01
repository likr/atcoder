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
        mut a: [usize; n],
    }
    a.dedup();
    let n = a.len();

    let mut b = vec![];
    if n > 1 && a[0] < a[1] {
        b.push(a[0]);
    }
    for i in 1..n - 1 {
        if (a[i - 1] < a[i] && a[i] > a[i + 1]) || (a[i - 1] > a[i] && a[i] < a[i + 1]) {
            b.push(a[i]);
        }
    }
    if n > 1 && a[n - 2] < a[n - 1] {
        b.push(a[n - 1]);
    }
    // eprintln!("{:?}", a);
    // eprintln!("{:?}", b);

    let mut money = 1000;
    for i in (0..b.len()).step_by(2) {
        money += (b[i + 1] - b[i]) * (money / b[i]);
    }
    println!("{}", money);
}

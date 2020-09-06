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
        x: usize,
        w: [usize; n],
    }
    if n == 1 {
        if w[0] == x {
            println!("1");
        } else {
            println!("0");
        }
        return;
    }

    let m1 = n / 2;
    let m2 = n - m1;
    let w1 = (0..m1).map(|i| w[i]).collect::<Vec<usize>>();
    let w2 = (m1..n).map(|i| w[i]).collect::<Vec<usize>>();

    let mut states1 = HashMap::new();
    for x in 0..2usize.pow(m1 as u32) {
        let mut s = 0;
        for i in 0..m1 {
            if 1 << i & x > 0 {
                s += w1[i];
            }
        }
        *states1.entry(s).or_insert(0) += 1;
    }

    let mut states2 = HashMap::new();
    for x in 0..2usize.pow(m2 as u32) {
        let mut s = 0;
        for i in 0..m2 {
            if 1 << i & x > 0 {
                s += w2[i];
            }
        }
        *states2.entry(s).or_insert(0) += 1;
    }

    let mut result = 0usize;
    for (&w1, &c1) in states1.iter() {
        if w1 <= x {
            if let Some(&c2) = states2.get(&(x - w1)) {
                result += c1 * c2;
            }
        }
    }
    println!("{}", result);
}

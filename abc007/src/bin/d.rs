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

fn to_oct(n: usize) -> usize {
    let convert = [0, 1, 2, 3, 0, 4, 5, 6, 7, 0];
    let mut m = n;
    let mut n = vec![];
    while m > 0 {
        n.push(m % 10);
        m /= 10;
    }

    let mut flag = false;
    for i in (0..n.len()).rev() {
        if !flag {
            if n[i] == 4 {
                flag = true;
                n[i] = 3;
            } else if n[i] == 9 {
                flag = true;
                n[i] = 8;
            }
        } else {
            n[i] = 8;
        }
    }
    eprintln!("{:?}", n);

    let mut k = 0;
    let mut base = 1;
    for i in 0..n.len() {
        k += base * convert[n[i]];
        base *= 8;
    }
    eprintln!("{}", k);
    k
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", b - to_oct(b) - ((a - 1) - to_oct(a - 1)));
}

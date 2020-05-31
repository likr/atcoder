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
        mut a: [isize; n],
    }
    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    a.sort();
    let mut a_max = a.pop().unwrap();
    a.reverse();
    let mut a_min = a.pop().unwrap();

    let mut result = vec![];
    for &ai in &a {
        if ai < 0 {
            result.push((a_max, ai));
            a_max -= ai;
        } else {
            result.push((a_min, ai));
            a_min -= ai;
        }
    }
    result.push((a_max, a_min));
    println!("{}", a_max - a_min);

    for &(x, y) in &result {
        println!("{} {}", x, y);
    }
}

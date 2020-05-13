use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        mut a: [usize; 3],
    }
    let n = a.iter().sum::<usize>();
    let mut x = (1..=n).collect::<Vec<_>>();
    let mut result = 0;
    loop {
        let mut check = true;
        for i in 1..a[0] {
            if x[i - 1] >= x[i] {
                check = false;
            }
        }
        for i in 1..a[1] {
            if x[a[0] + i - 1] >= x[a[0] + i] {
                check = false;
            }
        }
        for i in 1..a[2] {
            if x[a[0] + a[1] + i - 1] >= x[a[0] + a[1] + i] {
                check = false;
            }
        }
        for i in 0..min(a[0], a[1]) {
            if x[i] >= x[a[0] + i] {
                check = false;
            }
        }
        for i in 0..min(a[1], a[2]) {
            if x[a[0] + i] >= x[a[0] + a[1] + i] {
                check = false;
            }
        }
        if check {
            result += 1;
        }
        if !x.next_permutation() {
            break;
        }
    }
    println!("{}", result);
}

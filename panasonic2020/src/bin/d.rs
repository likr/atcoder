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

fn f(n: usize, k: usize, x: &mut Vec<usize>, x_max: usize, chars: &Vec<char>) {
    if k == n {
        for j in 0..n {
            print!("{}", chars[x[j]]);
        }
        println!("");
    } else {
        for i in 0..=x_max + 1 {
            x[k] = i;
            f(n, k + 1, x, max(x_max, i), chars);
        }
    }
}

fn main() {
    input! {
        n: usize,
    }
    let chars = "abcdefghij".chars().collect::<Vec<_>>();
    let mut x = vec![0; n];
    f(n, 1, &mut x, 0, &chars);
}

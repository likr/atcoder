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
        x: [usize; 5],
    }
    let n = x.len();
    let mut nums = HashSet::new();
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                nums.insert(x[i] + x[j] + x[k]);
            }
        }
    }
    let mut nums = nums.iter().collect::<Vec<_>>();
    nums.sort();
    nums.reverse();
    println!("{}", nums[2]);
}

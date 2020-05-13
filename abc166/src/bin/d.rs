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
        x: isize,
    }
    let mut nums = vec![0isize];
    let mut i = 1isize;
    while i.pow(5u32) - nums[nums.len() - 1] < x {
        nums.push(i.pow(5u32));
        i += 1;
    }
    nums.push(i.pow(5u32));
    // eprintln!("{:?}", nums);

    let m = nums.len();
    for i in 0..m {
        for j in 0..m {
            if nums[i] - nums[j] == x {
                println!("{} {}", i, j);
                return;
            } else if nums[i] + nums[j] == x {
                println!("{} -{}", i, j);
                return;
            }
        }
    }
}

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
        k: usize,
    }
    if k <= 12 {
        println!("{}", k);
        return;
    }

    let mut nums = vec![vec![]; 11];
    nums[0].push(0);
    for d in 1..10 {
        nums[1].push(d);
    }
    nums[2].push(10);
    nums[2].push(11);
    nums[2].push(12);
    let mut ones = vec![0, 1, 10, 11, 12];
    let mut twos = vec![0, 2];

    let mut count = 12;
    let mut m = 2;
    loop {
        // eprintln!("{}", m);
        // for i in 0..nums.len() {
        //     eprintln!("{:?}", nums[i]);
        // }
        let base = 10usize.pow(m as u32 - 1);
        if m >= 3 {
            for i in 0..ones.len() {
                let num = ones[i];
                nums[m].push(base + num);
                ones.push(base + num);
                count += 1;
                if count == k {
                    println!("{}", nums[m].last().unwrap());
                    return;
                }
            }
            for i in 0..nums[m - 1].len() {
                let num = nums[m - 1][i];
                let e = num / (base / 10);
                if e == 2 {
                    nums[m].push(base + num);
                    ones.push(base + num);
                    count += 1;
                    if count == k {
                        println!("{}", nums[m].last().unwrap());
                        return;
                    }
                }
                if e > 2 {
                    break;
                }
            }
        }
        for d in 2..10 {
            for i in 0..nums[m - 1].len() {
                let num = nums[m - 1][i];
                if (d as i64 - num as i64 / (base / 10) as i64).abs() <= 1 {
                    nums[m].push(d * base + num);
                    if d == 2 {
                        twos.push(d * base + num);
                    }
                    count += 1;
                    if count == k {
                        println!("{}", nums[m].last().unwrap());
                        return;
                    }
                }
            }
        }
        m += 1;
    }
}

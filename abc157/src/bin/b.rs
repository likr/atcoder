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
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }
    let mut flag = vec![vec![false; 3]; 3];
    for &bk in &b {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == bk {
                    flag[i][j] = true;
                }
            }
        }
    }
    for i in 0..3 {
        if (0..3).all(|j| flag[i][j]) {
            println!("Yes");
            return;
        }
    }
    for j in 0..3 {
        if (0..3).all(|i| flag[i][j]) {
            println!("Yes");
            return;
        }
    }
    if flag[0][0] && flag[1][1] && flag[2][2] {
        println!("Yes");
        return;
    }
    if flag[2][0] && flag[1][1] && flag[0][2] {
        println!("Yes");
        return;
    }
    println!("No");
}

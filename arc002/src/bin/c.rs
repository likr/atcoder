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
        c: Chars,
    }
    let chars = ['A', 'B', 'X', 'Y'];
    let mut commands = vec![];
    for &c1 in &chars {
        for &c2 in &chars {
            commands.push([c1, c2]);
        }
    }
    let m = commands.len();
    let mut mask = vec![false; n];
    let mut result = INF;
    for i in 0..m {
        for j in 0..m {
            let mut count = 0;
            for k in 0..n {
                mask[k] = false;
            }
            let s1 = commands[i];
            for k in 1..n {
                if !mask[k - 1] && c[k - 1] == s1[0] && c[k] == s1[1] {
                    count += 1;
                    mask[k - 1] = true;
                    mask[k] = true;
                }
            }
            let s2 = commands[j];
            for k in 1..n {
                if !mask[k - 1] && !mask[k] && c[k - 1] == s2[0] && c[k] == s2[1] {
                    count += 1;
                    mask[k - 1] = true;
                    mask[k] = true;
                }
            }
            for k in 0..n {
                if !mask[k] {
                    count += 1;
                }
            }
            result = min(result, count);
        }
    }
    println!("{}", result);
}

use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      s: [Chars; 2],
    }
    let mut result;
    let mut index;
    let mut prev_vertical;
    if s[0][0] == s[1][0] {
        result = 3;
        index = 1;
        prev_vertical = true;
    } else {
        result = 6;
        index = 2;
        prev_vertical = false;
    }
    while index < n {
        if s[0][index] == s[1][index] {
            if prev_vertical {
                result = result * 2 % M;
            }
            prev_vertical = true;
            index += 1;
        } else {
            if prev_vertical {
                result = result * 2 % M;
            } else {
                result = result * 3 % M;
            }
            prev_vertical = false;
            index += 2;
        }
    }
    println!("{}", result);
}

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
        s: Chars,
    }
    let n = s.len();
    let mut c = vec![];
    let mut i = 0;
    while i < n {
        if s[i] == '2' {
            let mut j = 0;
            while i + j < n && ((j % 2 == 0 && s[i + j] == '2') || (j % 2 == 1 && s[i + j] == '5'))
            {
                j += 1;
            }
            if j > 0 {
                c.push(j / 2);
            }
            i += j;
        } else {
            i += 1;
        }
    }
    let mut result = 0;
    for &ci in &c {
        result += ci * (ci + 1) / 2;
    }
    println!("{}", result);
}

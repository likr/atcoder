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
        mut s: Chars,
    }
    let n = s.len();
    let mut t = vec![];
    let mut stack = 0;
    for i in 1..n - 1 {
        match s[i] {
            '{' => {
                stack += 1;
            }
            '}' => {
                stack -= 1;
                if stack == 0 {
                    t.push('S');
                }
            }
            c => {
                if stack == 0 {
                    t.push(c);
                }
            }
        }
    }
    if n == 2 || t.iter().find(|&&c| c == ':').is_some() {
        println!("dict");
    } else {
        println!("set");
    }
}

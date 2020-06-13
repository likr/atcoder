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

fn rec(s: &Vec<char>, start: usize, stop: usize) -> Vec<char> {
    let mut result = vec![];
    let mut stack = vec![];
    for i in start..stop {
        if s[i] == '(' {
            stack.push(i);
        } else if s[i] == ')' {
            let j = stack.pop().unwrap();
            if stack.is_empty() {
                let mut t = rec(s, j + 1, i);
                for &c in &t {
                    result.push(c);
                }
                t.reverse();
                for &c in &t {
                    result.push(c);
                }
            }
        } else if stack.is_empty() {
            result.push(s[i]);
        }
    }
    result
}

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", rec(&s, 0, s.len()).iter().collect::<String>());
}

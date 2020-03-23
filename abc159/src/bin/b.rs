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

fn is_palindrome(s: &Vec<char>, a: usize, b: usize) -> bool {
    // eprintln!("{:?}", s[a..=b].to_vec());
    for i in a..b {
        if s[i] != s[b - i] {
            return false;
        }
    }
    return true;
}

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    if is_palindrome(&s, 0, n - 1)
        && is_palindrome(&s, 0, (n - 1) / 2 - 1)
        && is_palindrome(&s, (n + 3) / 2 - 1, n - 1)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

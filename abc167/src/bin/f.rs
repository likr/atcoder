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
        s: [Chars; n],
    }
    let mut left = vec![0usize; n];
    let mut right = vec![0usize; n];
    for i in 0..n {
        let mut stack = 0;
        for &c in &s[i] {
            if c == '(' {
                stack += 1;
            } else {
                if stack == 0 {
                    left[i] += 1;
                } else {
                    stack -= 1;
                }
            }
        }
        right[i] += stack;
    }

    let mut parts = (0..n).map(|i| (left[i], right[i])).collect::<Vec<_>>();
    parts.sort_by_key(|&(li, ri)| (li, Reverse(ri)));
    let mut left = 0;
    for &(li, ri) in &parts {
        if ri > li {
            if li > left {
                eprintln!("1");
                println!("No");
                return;
            } else {
                left -= li;
                left += ri;
            }
        }
    }
    parts.sort_by_key(|&(li, ri)| (ri, Reverse(li)));
    let mut right = 0;
    for &(li, ri) in &parts {
        if ri < li {
            if ri > right {
                eprintln!("3");
                println!("No");
                return;
            } else {
                right -= ri;
                right += li;
            }
        }
    }
    for &(li, ri) in &parts {
        if ri == li {
            if li > left || ri > right {
                eprintln!("2");
                println!("No");
                return;
            }
        }
    }
    eprintln!("{} {}", left, right);
    if left == right {
        println!("Yes");
    } else {
        println!("No");
    }
}

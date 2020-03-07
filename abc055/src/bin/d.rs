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
        s: Chars,
    }
    for x in 0..4 {
        let mut answer = vec![' '; n];
        for i in 0..2 {
            if x & 1 << i > 0 {
                answer[i] = 'S';
            } else {
                answer[i] = 'W';
            }
        }
        for i in 2..n {
            if (answer[i - 1] == 'S' && s[i - 1] == 'o')
                || (answer[i - 1] == 'W' && s[i - 1] == 'x')
            {
                answer[i] = answer[i - 2];
            } else {
                if answer[i - 2] == 'S' {
                    answer[i] = 'W';
                } else {
                    answer[i] = 'S';
                }
            }
        }
        // eprintln!("{}", answer.iter().collect::<String>());
        if (answer[0] == 'S' && s[0] == 'o') || (answer[0] == 'W' && s[0] == 'x') {
            if answer[1] != answer[n - 1] {
                continue;
            }
        } else {
            if answer[1] == answer[n - 1] {
                continue;
            }
        }
        if (answer[n - 1] == 'S' && s[n - 1] == 'o') || (answer[n - 1] == 'W' && s[n - 1] == 'x') {
            if answer[0] != answer[n - 2] {
                continue;
            }
        } else {
            if answer[0] == answer[n - 2] {
                continue;
            }
        }
        println!("{}", answer.iter().collect::<String>());
        return;
    }
    println!("-1");
}

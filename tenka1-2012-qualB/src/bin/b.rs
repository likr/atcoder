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
        mut c: Chars,
    }
    let mut last = 0;
    while c.len() > 0 && c[c.len() - 1] == '_' {
        last += 1;
        c.pop();
    }
    c.reverse();
    let mut first = 0;
    while c.len() > 0 && c[c.len() - 1] == '_' {
        first += 1;
        c.pop();
    }
    c.reverse();
    for _ in 0..first {
        print!("_");
    }
    let n = c.len();
    if (0..n).any(|i| c[i] == '_') {
        if (0..n).any(|i| 'A' <= c[i] && c[i] <= 'Z') {
            for &ci in &c {
                print!("{}", ci);
            }
        } else {
            let words = c.iter().collect::<String>();
            let words = words.split('_').collect::<Vec<_>>();
            if words.iter().any(|w| w.is_empty()) {
                for &ci in &c {
                    print!("{}", ci);
                }
            } else {
                for w in &words {
                    for (i, c) in w.chars().enumerate() {
                        if i == 0 {
                            print!(
                                "{}",
                                (c as usize - 'a' as usize + 'A' as usize) as u8 as char
                            );
                        } else {
                            print!("{}", c);
                        }
                    }
                }
            }
        }
    } else {
        if (c[0] < 'A' || 'Z' < c[0]) && (0..n).any(|i| 'A' <= c[i] && c[i] <= 'Z') {
            for i in 0..n {
                if 'A' <= c[i] && c[i] <= 'Z' {
                    print!(
                        "_{}",
                        (c[i] as usize - 'A' as usize + 'a' as usize) as u8 as char
                    );
                } else {
                    print!("{}", c[i]);
                }
            }
        } else {
            for &ci in &c {
                print!("{}", ci);
            }
        }
    }
    for _ in 0..last {
        print!("_");
    }
    println!("");
}

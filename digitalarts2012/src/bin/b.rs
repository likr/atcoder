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
        s: String,
    }
    let d = s
        .chars()
        .map(|c| c as usize - 'a' as usize + 1)
        .collect::<Vec<usize>>();
    let m = d.iter().sum::<usize>();

    let mut t1 = vec![0; 20];
    for i in 0..m / 26 {
        t1[i] = 26;
    }
    if m / 26 < 20 {
        t1[m / 26] = m % 26;
    }

    let mut t2 = vec![0; 20];
    for i in 0..20 {
        if i < m % 20 {
            t2[i] = m / 20 + 1;
        } else {
            t2[i] = m / 20;
        }
    }
    t2.reverse();

    let t1 = t1
        .into_iter()
        .filter(|&d| d > 0)
        .map(|d| ((d - 1) + 'a' as usize) as u8 as char)
        .collect::<String>();
    let t2 = t2
        .into_iter()
        .filter(|&d| d > 0)
        .map(|d| ((d - 1) + 'a' as usize) as u8 as char)
        .collect::<String>();

    eprintln!("{} {} {}", m, t1, t2);
    if t1 == t2 {
        println!("NO");
    } else if t1 == s {
        println!("{}", t2);
    } else {
        println!("{}", t1);
    }
}

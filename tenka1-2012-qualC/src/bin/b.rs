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
    let mut cards = vec![];
    let mut i = 0;
    while i < s.len() {
        if s[i + 1] == '1' {
            cards.push((s[i], '0'));
            i += 1;
        } else {
            cards.push((s[i], s[i + 1]));
        }
        i += 2;
    }
    let target = "0JQKA".chars().collect::<HashSet<_>>();
    let trash = "SHDC"
        .chars()
        .map(|mark| {
            let mut count = 0;
            let mut trash = vec![];

            for i in 0..cards.len() {
                let (m, n) = cards[i];
                if m == mark && target.contains(&n) {
                    count += 1;
                } else {
                    trash.push((m, n));
                }
                if count == 5 {
                    break;
                }
            }
            trash
        })
        .min_by_key(|trash| trash.len())
        .unwrap();
    if trash.is_empty() {
        println!("0");
    } else {
        for &(m, n) in &trash {
            if n == '0' {
                print!("{}{}", m, 10);
            } else {
                print!("{}{}", m, n);
            }
        }
        println!("");
    }
}

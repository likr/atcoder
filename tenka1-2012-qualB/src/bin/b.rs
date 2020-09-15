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

fn camelcase(s: &Vec<char>) -> bool {
    if s.len() == 0 {
        return true;
    }
    if '0' <= s[0] && s[0] <= '9' {
        return false;
    }
    if 'A' <= s[0] && s[0] <= 'Z' {
        return false;
    }
    s.iter().all(|&c| c != '_')
}

fn snakecase(s: &Vec<char>) -> bool {
    if s.len() == 0 {
        return true;
    }
    if '0' <= s[0] && s[0] <= '9' {
        return false;
    }
    for i in 1..s.len() {
        if s[i - 1] == '_' && ('0' <= s[i] && s[i] <= '9') {
            return false;
        }
        if s[i - 1] == '_' && s[i] == '_' {
            return false;
        }
    }
    s.iter().all(|&c| c < 'A' || 'Z' < c)
}

fn to_snakecase(s: &Vec<char>) -> Vec<char> {
    let mut t = vec![];
    for &c in s {
        if 'A' <= c && c <= 'Z' {
            t.push('_');
            t.push((c as usize + 'a' as usize - 'A' as usize) as u8 as char);
        } else {
            t.push(c);
        }
    }
    t
}

fn to_camelcase(s: &Vec<char>) -> Vec<char> {
    let mut t = vec![];
    for i in 0..s.len() {
        if i > 1 && s[i - 1] == '_' {
            t.push((s[i] as usize + 'A' as usize - 'a' as usize) as u8 as char);
        } else if s[i] != '_' {
            t.push(s[i]);
        }
    }
    t
}

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

    let d = if camelcase(&c) {
        to_snakecase(&c)
    } else if snakecase(&c) {
        to_camelcase(&c)
    } else {
        c
    };
    for &ci in &d {
        print!("{}", ci);
    }

    for _ in 0..last {
        print!("_");
    }
    println!("");
}

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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        s:Chars,
    }
    let mut paren = HashMap::new();
    let mut stack = vec![];
    for i in 0..n {
        if s[i] == '(' {
            stack.push(i);
        } else if s[i] == ')' {
            if let Some(j) = stack.pop() {
                paren.insert(i, j);
                paren.insert(j, i);
            }
        }
    }
    debug!(paren);
    let mut count = 0;
    let mut ans = vec![];
    for i in 0..n {
        debug!(i, count);
        if s[i] == '(' && paren.contains_key(&i) {
            count += 1;
        } else if s[i] == ')' && paren.contains_key(&i) {
            count -= 1;
        } else if count == 0 {
            ans.push(s[i]);
        }
    }
    if ans.len() > 0 {
        println!("{}", ans.iter().collect::<String>());
    }
}

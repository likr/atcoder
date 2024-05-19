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

fn f(
    l: usize,
    r: usize,
    h: usize,
    s: &Vec<char>,
    t: &mut Vec<char>,
    paren: &HashMap<usize, usize>,
) {
    if l > r {
        return;
    }
    if h % 2 == 0 {
        let mut i = l;
        loop {
            if s[i] == '(' {
                f(i + 1, paren[&i] - 1, h + 1, s, t, paren);
                i = paren[&i];
            } else if s[i] != ')' {
                t.push(s[i]);
            }
            if i == r {
                break;
            }
            i += 1;
        }
    } else {
        let mut i = r;
        loop {
            if s[i] == ')' {
                f(paren[&i] + 1, i - 1, h + 1, s, t, paren);
                i = paren[&i];
            } else if s[i] != '(' {
                t.push(s[i]);
            }
            if i == l {
                break;
            }
            i -= 1;
        }
    }
}

fn main() {
    input! {
        mut s: Chars,
    }
    let n = s.len();
    let mut h = vec![0; n];
    let mut stack = vec![];
    let mut paren = HashMap::new();
    for i in 0..n {
        if s[i] == '(' {
            stack.push(i);
        }
        h[i] = stack.len();
        if s[i] == ')' {
            let j = stack.pop().unwrap();
            paren.insert(i, j);
            paren.insert(j, i);
        }
    }
    for i in 0..n {
        if h[i] % 2 == 1 && s[i] != '(' && s[i] != ')' {
            if s[i].is_uppercase() {
                s[i] = s[i].to_ascii_lowercase();
            } else {
                s[i] = s[i].to_ascii_uppercase();
            }
        }
    }
    let mut ans = vec![];
    f(0, n - 1, 0, &s, &mut ans, &paren);
    if ans.len() > 0 {
        println!("{}", ans.iter().collect::<String>());
    }
}

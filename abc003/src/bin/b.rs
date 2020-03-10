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
        t: Chars,
    }
    let wildcard = "atcoder".chars().collect::<HashSet<char>>();
    let n = s.len();
    for i in 0..n {
        if s[i] != t[i]
            && !(s[i] == '@' && wildcard.contains(&t[i]))
            && !(t[i] == '@' && wildcard.contains(&s[i]))
        {
            println!("You will lose");
            return;
        }
    }
    println!("You can win");
}

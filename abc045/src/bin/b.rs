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
        sa: Chars,
        sb: Chars,
        sc: Chars,
    }
    let sa = sa
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<VecDeque<_>>();
    let sb = sb
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<VecDeque<_>>();
    let sc = sc
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<VecDeque<_>>();
    let mut s = [sa, sb, sc];
    let mut c = 0;
    loop {
        if let Some(d) = s[c].pop_front() {
            c = d;
        } else {
            println!("{}", ('A' as usize + c) as u8 as char);
            return;
        }
    }
}

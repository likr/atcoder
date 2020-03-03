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
        q: usize,
        query: [(usize, usize, String); q],
    }
    let s = s
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let mut acc = vec![vec![0; 26]; n + 1];
    for i in 1..=n {
        for j in 0..26 {
            acc[i][j] = acc[i - 1][j];
        }
        acc[i][s[i - 1]] += 1;
    }
    // eprintln!("{:?}", acc);
    let mut plus = vec![BTreeSet::new(); 26];
    let mut minus = vec![BTreeSet::new(); 26];
    'outer: for (t, i, c) in query {
        // eprintln!("{} {} {}", t, i, c);
        if t == 1 {
            let c = c.chars().nth(0).unwrap() as usize - 'a' as usize;
            for d in 0..26 {
                if plus[d].contains(&i) {
                    plus[d].remove(&i);
                    plus[c].insert(i);
                    continue 'outer;
                }
            }
            let d = s[i - 1];
            if c != d {
                plus[c].insert(i);
                minus[d].insert(i);
            }
        } else {
            let l = i;
            let r = c.parse::<usize>().ok().unwrap();
            let mut count = 0;
            for j in 0..26 {
                let pr = plus[j].range(..=r).count();
                let mr = minus[j].range(..=r).count();
                let pl = plus[j].range(..l).count();
                let ml = minus[j].range(..l).count();
                let pj = pr - pl;
                let mj = mr - ml;
                let cj = acc[r][j] - acc[l - 1][j];
                // eprintln!("{} {} {} {}", (j + 'a' as usize) as u8 as char, pj, mj, cj);
                if cj + pj - mj > 0 {
                    count += 1;
                }
            }
            println!("{}", count);
        }
    }
}

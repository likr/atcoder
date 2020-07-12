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
        first: String,
        last: String,
        n: usize,
        mut s: [String; n],
    }
    s.push(first.clone());
    s.push(last.clone());
    s.sort();
    s.dedup();
    let n = s.len();
    let mut queue = VecDeque::new();
    queue.push_back(&first);
    let mut parent = HashMap::new();
    parent.insert(&first, None);
    while let Some(t) = queue.pop_front() {
        for i in 0..n {
            if parent.contains_key(&s[i]) {
                continue;
            }
            let mut diff = 0;
            for (a, b) in t.chars().zip(s[i].chars()) {
                if a != b {
                    diff += 1;
                }
            }
            if diff == 1 {
                queue.push_back(&s[i]);
                parent.insert(&s[i], Some(t));
            }
        }
    }
    if !parent.contains_key(&last) {
        println!("-1");
        return;
    }
    let mut path = vec![];
    let mut p = &last;
    loop {
        if let Some(q) = parent[&p] {
            if *q == first {
                break;
            }
            path.push(q.clone());
            p = &q;
        } else {
            break;
        }
    }
    path.reverse();
    println!("{}", path.len());
    println!("{}", first);
    for p in &path {
        println!("{}", p);
    }
    println!("{}", last);
}

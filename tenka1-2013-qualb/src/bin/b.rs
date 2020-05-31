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
        q: usize,
        l: usize,
    }
    let mut stack = vec![];
    let mut size = 0;
    for _ in 0..q {
        input! {
            query: String,
        }
        match &*query {
            "Push" => {
                input! {
                    n: usize,
                    m: isize,
                }
                stack.push((n, m));
                size += n;
                if size > l {
                    println!("FULL");
                    return;
                }
            }
            "Pop" => {
                input! {
                    mut n : usize,
                }
                if n > size {
                    println!("EMPTY");
                    return;
                }
                loop {
                    let (a, b) = stack.pop().unwrap();
                    size -= a;
                    if n <= a {
                        if a - n > 0 {
                            stack.push((a - n, b));
                            size += a - n;
                        }
                        break;
                    } else {
                        n -= a;
                    }
                }
            }
            "Top" => {
                if size == 0 {
                    println!("EMPTY");
                    return;
                }
                let (a, b) = stack.pop().unwrap();
                println!("{}", b);
                stack.push((a, b));
            }
            "Size" => {
                println!("{}", size);
            }
            _ => {}
        }
    }
    println!("SAFE");
}

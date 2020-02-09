use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      h: usize,
      w: usize,
    }
    if h == 1 {
        if w % 7 == 0 {
            println!("Yes");
            for _ in 0..w / 7 {
                print!("5555522");
            }
            println!("");
        } else if w % 7 == 2 {
            println!("Yes");
            print!("22");
            for _ in 0..w / 7 {
                print!("5555522");
            }
            println!("");
        } else if w % 7 == 5 {
            println!("Yes");
            print!("55555");
            for _ in 0..w / 7 {
                print!("2255555");
            }
            println!("");
        } else {
            println!("No");
        }
    } else if h == 2 {
        if w % 7 == 0 {
            println!("Yes");
            for _ in 0..w / 7 {
                print!("2555255");
            }
            println!("");
            for _ in 0..w / 7 {
                print!("2552555");
            }
            println!("");
        } else if w % 7 == 1 {
            println!("Yes");
            print!("2");
            for _ in 0..w / 7 {
                print!("5552552");
            }
            println!("");
            print!("2");
            for _ in 0..w / 7 {
                print!("5525552");
            }
            println!("");
        } else if w % 7 == 6 {
            println!("Yes");
            print!("555255");
            for _ in 0..w / 7 {
                print!("2555255");
            }
            println!("");
            print!("552555");
            for _ in 0..w / 7 {
                print!("2552555");
            }
            println!("");
        } else {
            println!("No");
        }
    } else if h == 3 && w == 3 {
        println!("Yes");
        println!("522");
        println!("555");
        println!("522");
    } else if w == 2 {
        if h % 7 == 0 {
            println!("Yes");
            for _ in 0..h / 7 {
                println!("22");
                println!("55");
                println!("55");
                println!("52");
                println!("25");
                println!("55");
                println!("55");
            }
        } else if h % 7 == 1 {
            println!("Yes");
            println!("22");
            for _ in 0..h / 7 {
                println!("55");
                println!("55");
                println!("52");
                println!("25");
                println!("55");
                println!("55");
                println!("22");
            }
        } else if h % 7 == 6 {
            println!("Yes");
            println!("55");
            println!("55");
            println!("52");
            println!("25");
            println!("55");
            println!("55");
            for _ in 0..h / 7 {
                println!("22");
                println!("55");
                println!("55");
                println!("52");
                println!("25");
                println!("55");
                println!("55");
            }
        } else {
            println!("No");
        }
    } else if w == 1 {
        if h % 7 == 0 {
            println!("Yes");
            for _ in 0..h / 7 {
                println!("5");
                println!("5");
                println!("5");
                println!("5");
                println!("5");
                println!("2");
                println!("2");
            }
        } else if h % 7 == 2 {
            println!("Yes");
            println!("2");
            println!("2");
            for _ in 0..h / 7 {
                println!("5");
                println!("5");
                println!("5");
                println!("5");
                println!("5");
                println!("2");
                println!("2");
            }
        } else if h % 7 == 5 {
            println!("Yes");
            println!("5");
            println!("5");
            println!("5");
            println!("5");
            println!("5");
            for _ in 0..h / 7 {
                println!("2");
                println!("2");
                println!("5");
                println!("5");
                println!("5");
                println!("5");
                println!("5");
            }
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}

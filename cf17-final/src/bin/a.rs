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
        s: String,
    }
    let r = match &*s {
        "AKIHABARA" => "YES",
        "AKIHABAR" => "YES",
        "AKIHABRA" => "YES",
        "AKIHABR" => "YES",
        "AKIHBARA" => "YES",
        "AKIHBAR" => "YES",
        "AKIHBRA" => "YES",
        "AKIHBR" => "YES",
        "KIHABARA" => "YES",
        "KIHABAR" => "YES",
        "KIHABRA" => "YES",
        "KIHABR" => "YES",
        "KIHBARA" => "YES",
        "KIHBAR" => "YES",
        "KIHBRA" => "YES",
        "KIHBR" => "YES",
        _ => "NO",
    };
    println!("{}", r);
}

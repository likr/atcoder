use chrono::prelude::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use time::Duration;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        s: String,
    }
    let y = s[0..4].parse::<usize>().unwrap();
    let m = s[5..7].parse::<usize>().unwrap();
    let d = s[8..10].parse::<usize>().unwrap();
    let mut date = Utc.ymd(y as i32, m as u32, d as u32);
    while date.year() as u32 % date.month() != 0
        || (date.year() as u32 / date.month()) % date.day() != 0
    {
        date = date + Duration::days(1);
    }
    println!("{:04}/{:02}/{:02}", date.year(), date.month(), date.day());
}

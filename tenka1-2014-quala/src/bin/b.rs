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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
    #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Event {
    Back(usize),
    ChargeThrow,
    Damage(usize),
    Throw,
}

fn process(e: Event, kabu: &mut usize, combo: &mut usize, charge: &mut bool, result: &mut usize) {
    match e {
        Event::Back(n) => *kabu += n,
        Event::ChargeThrow => {
            *kabu -= 3;
            *charge = false;
        }
        Event::Damage(d) => {
            *combo += 1;
            *result += d;
        }
        Event::Throw => {
            *kabu -= 1;
        }
    }
}

fn d(base: usize, combo: usize) -> usize {
    base + base / 10 * (combo / 10)
}

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut kabu = 5;
    let mut events = BinaryHeap::new();
    let mut combo = 0usize;
    let mut charge = false;
    let mut result = 0usize;

    for t in 0..n {
        if !charge {
            match s[t] {
                'N' => {
                    if kabu >= 1 {
                        events.push((Reverse(t), Event::Throw));
                        events.push((Reverse(t + 1), Event::Damage(d(10, combo))));
                        events.push((Reverse(t + 6), Event::Back(1)));
                    }
                }
                'C' => {
                    if kabu >= 3 {
                        charge = true;
                        events.push((Reverse(t + 2), Event::ChargeThrow));
                        events.push((Reverse(t + 3), Event::Damage(d(50, combo))));
                        events.push((Reverse(t + 8), Event::Back(3)));
                    }
                }
                _ => {}
            }
        }
        while let Some(&(Reverse(t0), e)) = events.peek() {
            if t0 > t {
                break;
            }
            process(e, &mut kabu, &mut combo, &mut charge, &mut result);
            events.pop();
        }
        debug!(events);
    }
    while let Some((_, e)) = events.pop() {
        process(e, &mut kabu, &mut combo, &mut charge, &mut result);
    }
    println!("{}", result);
}

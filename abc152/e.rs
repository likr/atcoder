use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

const M : usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max = a[0];
    for i in 1..n {
        if a[i] > max {
            max = a[i];
        }
    }

    let mut factors = HashMap::new();
    for i in 0..n {
        let mut ai = a[i];
        let target = (ai as f64).sqrt() as usize;
        for j in 2..target + 1 {
            let mut count = 0;
            while ai % j == 0 {
                count += 1;
                ai /= j
            }
            if count > 0 {
                if !factors.contains_key(&j) || count > factors[&j] {
                    factors.insert(j, count);
                }
            }
            if ai == 1 {
                break;
            }
        }
        if ai > 1 {
            if !factors.contains_key(&ai) {
                factors.insert(ai, 1);
            }
        }
    }
    // println!("{:?}", factors);

    let mut lcm = 1;
    for (p, count) in factors {
        for _ in 0..count {
            lcm = lcm * p % M;
        }
    }
    // println!("{}", lcm);

    let mut result = 0;
    for i in 0..n {
        result = (result + lcm * inv(a[i]) % M) % M;
    }
    println!("{}", result);
}

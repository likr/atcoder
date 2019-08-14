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

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    let mut ones = vec![];
    let mut zeros = vec![];
    if s[0] == '0' {
        ones.push(0i32);
    }
    let mut i = 0;
    while i < n {
        let c0 = s[i];
        let j = i;
        while i < n && c0 == s[i] {
            i += 1;
        }
        if c0 == '0' {
            zeros.push((i - j) as i32);
        } else {
            ones.push((i - j) as i32);
        }
    }
    if s[n - 1] == '0' {
        ones.push(0);
    }
    zeros.push(0);
    let k = std::cmp::min(k, ones.len());
    let mut sum = 0i32;
    for i in 0..k {
        sum += ones[i];
        sum += zeros[i];
    }
    if k < ones.len() {
        sum += ones[k];
    }
    let mut max = sum;
    for i in 1..ones.len() - k {
        sum -= ones[i - 1];
        sum -= zeros[i - 1];
        sum += ones[i + k];
        sum += zeros[i + k - 1];
        if sum > max {
            max = sum;
        }
    }
    println!("{}", max);
}

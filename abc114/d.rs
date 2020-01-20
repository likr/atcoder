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
    }
    let mut factors = vec![0; n + 1];
    for i in 1..n + 1 {
        let t = (i as f64).sqrt() as usize;
        let mut m = i;
        for d in 2..t + 1 {
            while m % d == 0 {
                factors[d] += 1;
                m /= d;
            }
        }
        if m > 1 {
            factors[m] += 1;
        }
    }
    let mut count3 = 0;
    let mut count5 = 0;
    let mut count15 = 0;
    let mut count25 = 0;
    let mut count75 = 0;
    for i in 0..n + 1 {
        if factors[i] + 1 >= 3 {
            count3 += 1;
        }
        if factors[i] + 1 >= 5 {
            count5 += 1;
        }
        if factors[i] + 1 >= 15 {
            count15 += 1;
        }
        if factors[i] + 1 >= 25 {
            count25 += 1;
        }
        if factors[i] + 1 >= 75 {
            count75 += 1;
        }
    }
    let mut result = 0;
    if count3 >= 3 && count5 >= 2 {
        result += count5 * (count5 - 1) / 2 * (count3 - count5);
    }
    if count5 >= 3 {
        result += count5 * (count5 - 1) / 2 * (count5 - 2);
    }
    if count15 >= 1 {
        result += count15 * (count5 - 1);
    }
    if count25 >= 1 {
        result += count25 * (count3 - 1);
    }
    if count75 >= 1 {
        result += count75;
    }
    println!("{}", result);
}

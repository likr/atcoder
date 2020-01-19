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

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        return gcd(b, a);
    }
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut div_count = 0;
    let mut t = a[0];
    while t % 2 == 0 {
        div_count += 1;
        t /= 2;
    }
    for i in 1..n {
        let mut ai = a[i];
        let mut div_count2 = 0;
        while ai % 2 == 0 {
            div_count2 += 1;
            ai /= 2;
        }
        if div_count != div_count2 {
            println!("0");
            return;
        }
    }
    let mut lcm = 1;
    for i in 0..n {
        let ai = a[i] / 2;
        lcm = lcm * ai / gcd(lcm, ai);
        if lcm > m {
            println!("0");
            return;
        }
    }
    println!("{}", (m - lcm) / (lcm * 2) + 1);
}

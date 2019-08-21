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

fn nth(n: usize) -> usize {
    let mut i = n;
    let v : Vec<usize> = vec![0, 3, 5, 7];
    let mut x = 0;
    let mut d = 1;
    while i > 0 {
        let j = i % 4;
        x += d * v[j];
        i /= 4;
        d *= 10;
    }
    x
}

fn check(x: usize) -> bool {
    let mut x = x;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;
    while x > 0 {
        match x % 10 {
            3 => c1 += 1,
            5 => c2 += 1,
            7 => c3 += 1,
            _ => return false,
        }
        x /= 10;
    }
    c1 > 0 && c2 > 0 && c3 > 0
}

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    let mut i = 0;
    loop {
        let x = nth(i);
        if x > n {
            break;
        }
        if check(x) {
            ans += 1;
        }
        i += 1;
    }
    println!("{}", ans);
}

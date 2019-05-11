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

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}


fn main() {
    input!{
        n: usize,
        s: [String; n],
    }
    let mut count = 0;
    for si in s.iter() {
        let m = si.matches("AB").collect::<Vec<_>>();
        count += m.len();
    }
    let mut a_only : usize = 0;
    let mut b_only : usize = 0;
    let mut a_and_b = 0;
    for si in s.iter() {
        if si.starts_with("B") && si.ends_with("A") {
            a_and_b += 1;
        } else {
            if si.starts_with("B") {
                b_only += 1;
            }
            if si.ends_with("A") {
                a_only += 1;
            }
        }
    }
    if a_and_b > 0 {
        count += a_and_b - 1;
        if b_only > 0 {
            count += 1;
            b_only -= 1;
        }
        if a_only > 0 {
            count += 1;
            a_only -= 1;
        }
    }
    count += if a_only > b_only { b_only } else { a_only };
    println!("{}", count);
}


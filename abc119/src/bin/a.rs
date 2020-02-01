use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let y: usize = s[0..4].parse().unwrap();
    let m: usize = s[5..7].parse().unwrap();
    let d: usize = s[8..10].parse().unwrap();
    if (y, m, d) > (2019, 4, 30) {
        println!("TBD");
    } else {
        println!("Heisei");
    }
}

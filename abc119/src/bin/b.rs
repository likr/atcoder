use proconio::input;

fn main() {
    input! {
      n: usize,
      xu: [(f64, String); n],
    }
    let btc = 380000.;
    let mut sum = 0.;
    for (xi, ui) in xu {
        if ui == "JPY" {
            sum += xi;
        } else {
            sum += btc * xi;
        }
    }
    println!("{}", sum);
}

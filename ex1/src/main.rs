fn calc(x: i32, f: f64) -> i32 {
    let z = x as f64 * f + 5.0;
    let y = z * 3.0;
    y.round() as i32
}

fn main() {
    let mut r = 10;
    let arg = 22;
    println!("r={}", r);
    r = calc(arg, std::f64::consts::PI);
    println!("r={}", r);
}

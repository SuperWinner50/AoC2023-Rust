static INPUT: &str = include_str!("input.txt");

fn p1() -> u64 {
    let (time, distance) = INPUT.split_once("\n").unwrap();
    let time = time.split_whitespace().filter_map(|c| c.parse::<f32>().ok());
    let distance = distance.split_whitespace().filter_map(|c| c.parse::<f32>().ok());

    time.zip(distance).map(|(t, d)| {
        let x = (t.powi(2) - 4.0 * d).sqrt();
        let f = ((t - x) / 2.0).rem_euclid(1.0);
        (x + f).ceil() as u64 - 1

    }).product()
}

fn p2() -> u64 {
    let (time, distance) = INPUT.split_once("\n").unwrap();
    let t = time.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<f64>().unwrap();
    let d = distance.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<f64>().unwrap();

    let x = (t.powi(2) - 4.0 * d).sqrt();
    let f = ((t - x) / 2.0).rem_euclid(1.0);
    (x + f).ceil() as u64 - 1
}

fn main() {
    println!("{}", p2());
}

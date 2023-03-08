use std::env;

fn main() {
    let radius: i64 = env::args().nth(1).map(|arg| arg.parse::<i64>().unwrap()).unwrap_or(100);
    let area = (0..radius).reduce(|i, x| {
        i + (0..radius).reduce(|j, y| {
            // println!("{}", ((x.pow(2) + y.pow(2)) as f64).sqrt() < RADIUS as f64);
            if (((radius - x).abs().pow(2) 
               + (radius - y).abs().pow(2)) as f64).sqrt() <= radius as f64
                { j + 1 } else { j }
        }).unwrap()
    }).unwrap();

    // a = pi*r^2
    let pi = area as f64 * 4f64 / radius.pow(2) as f64;
    println!("pi: {pi}");
    println!("area: {area}");
}

use num::Complex;
use std::f64::consts::PI;
const ZERO_THRESHOLD: f64 = 1e-10;

fn main() {
    let x = Complex::new(1.0, 0.0);
    let y = Complex::new(4.0, 0.0);
    let test = vec![y, x, x, x];
    dft(&test);
}

pub fn dft(numbers: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
  let mut transformed_vec: Vec<Complex<f64>> = Vec::new();

  for idx in 0..numbers.len() {
    let mut transformed = numbers.iter().enumerate().fold(Complex::new(0.0, 0.0), |acc, (i, v)| {
      let x = -1.0 * 2.0 * PI / numbers.len() as f64 * idx as f64 * i as f64;
      let re = x.cos();
      let im = x.sin();

      return acc + Complex::new(re, im) * v;
    });

    transformed = match (transformed.re < ZERO_THRESHOLD, transformed.im < ZERO_THRESHOLD) {
      (true, true) => Complex::new(0.0, 0.0),
      (true, false) => Complex::new(0.0, transformed.im),
      (false, true) => Complex::new(transformed.re, 0.0),
      (false, false) => transformed,
    };

    transformed_vec.push(transformed);
  }

  println!("{:?}", transformed_vec);

  return transformed_vec;
}

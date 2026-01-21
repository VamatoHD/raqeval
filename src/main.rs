use reval::Rational;

fn main() {
    let a = Rational::new(1, 2, false).unwrap();
    let b = Rational::new(3, 4, true).unwrap();
    println!("{} + {}", &a, &b);
    println!("{}", a + b)
}

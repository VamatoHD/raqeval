use reval;

fn main() {
    let res = reval::parse("(1 + 2) * (3 - 1) / 2").unwrap();
    println!("{:?}", &res);
    println!("{}", &res);
    println!("{}", res.reduce());
}

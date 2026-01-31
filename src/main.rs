use reval;

fn main() {
    let res = reval::parse("2 * 2 ^ 3 + 3 - 2 * 10").unwrap();
    println!("{:?}", &res);
    println!("{}", res)
}

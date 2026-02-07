use reval;

fn main() {
    let f = reval::parse("2 * x", Some(&vec!["x"]), None).unwrap();

    let mut ctx = reval::Ctx::new();
    ctx.add_func("f".to_string(), f);

    let res = reval::parse("f(f(4)) + 1", Some(&vec!["x"]), Some(&vec!["f"])).unwrap();

    println!("{:?}", &res);
    println!("{}", &res);
    println!("{}", res.reduce(&ctx).unwrap());
}

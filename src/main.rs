use reval;

fn main() {
    let f = reval::parse_func("f(x) =  x + 1").unwrap();
    let g = reval::parse_func("g(y) =  2*y + 1").unwrap();

    let mut ctx = reval::Ctx::new();
    ctx.add_func(f);
    ctx.add_func(g);

    let res = reval::parse("g(f(2))", Some(&ctx)).unwrap();

    println!("{:?}", &res);
    println!("{}", &res);
    println!("{}", res.reduce(&ctx).unwrap());
}

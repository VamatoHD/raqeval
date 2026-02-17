use raqeval;

fn main() {
    let f = raqeval::parse_func("f(x) =  x + 1").unwrap();
    let g = raqeval::parse_func("g(y) =  2*y + 1").unwrap();

    let mut ctx = raqeval::Ctx::new();
    ctx.add_func(f).unwrap();
    ctx.add_func(g).unwrap();

    let res = raqeval::parse("g(f(2))").unwrap();

    println!("{:?}", &res);
    println!("{}", &res);
    println!("{}", res.reduce(&ctx).unwrap());
}

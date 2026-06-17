use raqeval;

fn main() {
    let f = raqeval::parse_func("f(x,y) =  g(3 + x, y) + 1 + h()").unwrap();
    let g = raqeval::parse_func("g(y,z) =  2*y + z + 1").unwrap();
    let h = raqeval::parse_func("h() =  1 + 1 + 1 + 1 + g(10, 2)").unwrap();

    let mut ctx = raqeval::Ctx::new();
    ctx.add_func(f).unwrap();
    ctx.add_func(g).unwrap();
    ctx.add_func(h).unwrap();

    let res = raqeval::parse("f(1,2)").unwrap();

    dbg!(&res);
    println!("{} = {}", &res, res.reduce(&ctx).unwrap());
}

use reval::Lexer;

fn main() {
    println!("{:?}", Lexer::new("10+10000/ 100"))
}

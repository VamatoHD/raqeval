use reval::Lexer;

fn main() {
    println!("{:?}", Lexer::new("100.0/50.1+2.5"))
}

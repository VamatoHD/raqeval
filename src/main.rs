use reval::Lexer;

fn main() {
    match Lexer::new(
        "sin(x)+cos(y)",
        Some(&vec!["x", "y"]),
        Some(&vec!["sin", "cos"]),
    ) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
}

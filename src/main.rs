use reval::{Lexer, Token};

fn main() {
    let mut l = match Lexer::new(
        "sin(x)+cos(y)",
        Some(&vec!["x", "y"]),
        Some(&vec!["sin", "cos"]),
    ) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut next = l.next();
    while !matches!(next, Token::Eof) {
        println!("{:?} -> {:?}", next, l.peek());
        next = l.next()
    }
    println!("{:?}", next);
}

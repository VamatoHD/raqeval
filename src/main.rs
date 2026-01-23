use reval::Lexer;

fn main() {
    println!(
        "{:?}",
        Lexer::new(
            "sin(x) * cos(y)",
            Some(&vec!["x", "y"]),
            Some(&vec!["sin", "cos"])
        )
    )
}

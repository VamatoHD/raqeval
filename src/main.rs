use reval::Lexer;

fn main() {
    println!(
        "{:?}",
        Lexer::new(
            "s i n(x) * cos(y) + 5 0 . 2",
            Some(&vec!["x", "y"]),
            Some(&vec!["sin", "cos"])
        )
    )
}

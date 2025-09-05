use symploke::lexer::lexer;


fn main() {
    let s: Vec<&str> = vec![
        "(+ 1 (abs -3))",
    ];

    for expr in s.iter() {
        let tokens = lexer(expr);
        println!("{:?}", tokens);
    }
}

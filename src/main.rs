use symploke::lexer;

fn main() {
    let s: Vec<&str> = vec![
        "(+ 1 2)",
        "((lambda (x) x) 5)",
        "(+ 1 2",
        "1 2 3)",
        "()",
        "(+ 1 (abs -3))",
        "(+ 1 (abs -3",
    ];

    for expr in s.iter() {
        let tokens = lexer(expr);
        println!("{:?}", tokens);
    }
}

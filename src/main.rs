mod lexer;
use lexer::Lexer;

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let test_input = String::from("(3 - 9 + (4 * 10)) / 10 + 3 * 100");

    let mut lexer = Lexer::init(test_input);

    let tokens = lexer.run();
    
    for token in tokens {
        println!("{:?}", token)
    }
}

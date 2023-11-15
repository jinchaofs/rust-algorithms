mod arithmetics;

use arithmetics::string::{atoi::Atoi, lexer_scanner::LexerScanner};
fn main() {
    let atoi = Atoi::new(" -22 asdb 33".to_string());
    println!("atoi res: {}", atoi.parse());

    let scanner = LexerScanner::new("\"23423423\" == \"222\"".to_string());
    let tokens = scanner.scan();
    println!("Tokens: {:#?}", tokens);
}

enum TokenType {
    Func(String),
    Oparan(String),
    Cparan(String),
    ReturnType(String),
    Var(String),
    Identifier(String),
    Semicolon(String),
}

struct Tokens {
    token_type: TokenType,
    value: u8,
}

pub fn tokenize (s: &String) -> Vec<Tokens> {
    let mut TokensList = Vec::new();

    let mut i = 0;
    while i < s.chars().count() {
        
    }
}
pub fn iterate(s: &String) {
    let _idk = Tokens{token_type: TokenType::Func(String::from("func")), value: 212};
    let mut i = 0;
    while i < s.chars().count() {
        let this = s.chars().nth(i);
        println!("{}th char: {:?}\n", i, this);
        i += 1;
    }
}
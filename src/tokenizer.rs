#[derive(Debug)]
enum TokenType {
    Func,
    Oparan,
    Cparan,
    Void,
    Let,
    Identifier(String),
    Semicolon,
    Shout,
    Ocurly,
    Ccurly,
    Eq,
    Comma,
    Intlit(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct Tokens {
    token_type: TokenType,
}

pub fn tokenize (s: &String) -> Vec<Tokens> {
    let mut tokens_list = Vec::new();
    let mut cache = String::new();

    for c in s.chars() {
        if (c == ('(') || c == (')') || c == (';') || c == (','))  && !cache.is_empty() {
            tokens_list.push(Tokens {
                token_type: TokenType::Identifier(cache.clone()),
            });
            cache.clear();
            cache.push(c);
            match cache.as_str() {
                "(" => {tokens_list.push(Tokens { token_type: TokenType::Oparan }); cache.clear();},
                ")" => {tokens_list.push(Tokens { token_type: TokenType::Cparan }); cache.clear();},
                ";" => {tokens_list.push(Tokens { token_type: TokenType::Semicolon }); cache.clear();},
                "," => {tokens_list.push(Tokens { token_type: TokenType::Comma }); cache.clear();},
                _ => {}
            }
            cache.clear();
            continue;
        }
        if c.is_ascii_whitespace(){
            if !cache.is_empty() {
                match cache.parse::<i32>() {
                    Ok(val) => {
                        tokens_list.push(Tokens {
                            token_type: TokenType::Intlit(val),
                        });
                    }
                    _ => {
                        tokens_list.push(Tokens {
                            token_type: TokenType::Identifier(cache.clone()),
                        });
                    }
                }
                cache.clear();
            }
            continue;
        }
        cache.push(c);

        match cache.as_str() {
            "func" => {tokens_list.push(Tokens { token_type: TokenType::Func }); cache.clear();},
            "=" => {tokens_list.push(Tokens { token_type: TokenType::Eq }); cache.clear();},
            "," => {tokens_list.push(Tokens { token_type: TokenType::Comma }); cache.clear();},
            "{" => {tokens_list.push(Tokens { token_type: TokenType::Ocurly}); cache.clear();},
            "}" => {tokens_list.push(Tokens { token_type: TokenType::Ccurly}); cache.clear();},
            "(" => {tokens_list.push(Tokens { token_type: TokenType::Oparan }); cache.clear();},
            ")" => {tokens_list.push(Tokens { token_type: TokenType::Cparan }); cache.clear();},
            "void" => {tokens_list.push(Tokens { token_type: TokenType::Void }); cache.clear();},
            "let" => {tokens_list.push(Tokens { token_type: TokenType::Let }); cache.clear();},
            ";" => {tokens_list.push(Tokens { token_type: TokenType::Semicolon }); cache.clear();},
            "shout" => {tokens_list.push(Tokens { token_type: TokenType::Shout }); cache.clear();},
            "+" => {tokens_list.push(Tokens { token_type: TokenType::Plus }); cache.clear();},
            "-" => {tokens_list.push(Tokens { token_type: TokenType::Minus }); cache.clear();},
            "*" => {tokens_list.push(Tokens { token_type: TokenType::Multiply }); cache.clear();},
            "/" => {tokens_list.push(Tokens { token_type: TokenType::Divide }); cache.clear();},
            _ => {}
        }
    }

    if !cache.is_empty() {
        tokens_list.push(Tokens {
            token_type: TokenType::Identifier(cache),
        });
    }

    tokens_list
}
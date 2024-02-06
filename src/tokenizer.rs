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
    Int,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tokens {
    token_type: TokenType,
}

fn token_match(tokens_list: &mut Vec<Tokens>, cache: &mut String) {
    match cache.as_str() {
        "func" => {tokens_list.push(Tokens { token_type: TokenType::Func }); cache.clear();},
        "int" => {tokens_list.push(Tokens { token_type: TokenType::Int }); cache.clear();},
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

fn to_num(tokens_list: &mut Vec<Tokens>, cache: &mut String) {
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
}

pub fn tokenize (s: &String) -> Vec<Tokens> {
    let mut tokens_list = Vec::new();
    let mut cache = String::new();

    for c in s.chars() {
        if (c == ('(') || c == (')') || c == (';') || c == (','))  && !cache.is_empty() {
            to_num(&mut tokens_list, &mut cache);
            cache.clear();
            cache.push(c);
            token_match(&mut tokens_list, &mut cache);
            cache.clear();
            continue;
        }
        if (cache.chars().nth(0) == Some('"') && c != '"') || (cache.chars().nth(0) == Some('"') && c == '"') {
            if c == '"' {
                cache.push(c);
                tokens_list.push(Tokens {token_type: TokenType::Identifier(cache.clone()),});
                cache.clear();
                continue;
            }
            else {
                cache.push(c);
                continue;
            }
        }
        if c.is_ascii_whitespace(){
            if !cache.is_empty() {
                to_num(&mut tokens_list, &mut cache);
                cache.clear();
            }
            continue;
        }
        cache.push(c);
        
        token_match(&mut tokens_list, &mut cache);
    }
    if !cache.is_empty() {
        tokens_list.push(Tokens {
            token_type: TokenType::Identifier(cache),
        });
    }

    return tokens_list;
}
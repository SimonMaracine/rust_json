use std::error::Error;
use std::fmt;

pub fn tokenize(contents: String) -> Result<Vec<Token>, &'static str> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_character: Option<char> = None;
    let mut current_position = -1;

    let contents_chars = contents.chars().collect();

    advance(&contents_chars, &mut current_character, &mut current_position);

    while let Some(character) = current_character {
        print!("{}", character);
        match character {
            ' ' | '\t' |
            '\n' | '\r' => (),

            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),

            '"' => {
                let string = build_string(&contents_chars, &mut current_character,
                                          &mut current_position);
                match string {
                    Ok(value) => tokens.push(Token::String(value)),
                    Err(message) => return Err(message)
                }
            }

            '0' | '1' |
            '2' | '3' |
            '4' | '5' |
            '6' | '7' |
            '8' | '9' => {
                let number = build_number(&contents_chars, &mut current_character,
                                          &mut current_position);
                match number {
                    Ok(value) => tokens.push(Token::Number(value)),
                    Err(message) => return Err(message)
                }

                // This is so that the character after the last number
                // character is evaluated (to not call advance())
                continue;
            }

            't' | 'f' | 'n' => {
                let keyword = build_keyword(&contents_chars, &mut current_character,
                                            &mut current_position);
                match keyword {
                    Ok(value) => tokens.push(Token::Keyword(value)),
                    Err(message) => return Err(message)
                }

                // This is so that the character after the last keyword
                // character is evaluated (to not call advance())
                continue;
            }

            _ => tokens.push(Token::Unidentified(character.to_string()))
        }
        advance(&contents_chars, &mut current_character, &mut current_position);
    }

    Ok(tokens)
}

fn advance(contents_chars: &Vec<char>, current_character: &mut Option<char>,
           current_position: &mut i32) {
    *current_position = *current_position + 1;
    if *current_position < contents_chars.len() as i32 {
        *current_character = Some(contents_chars[*current_position as usize]);
    } else {
        *current_character = None;
    }
}

fn build_string(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                current_position: &mut i32) -> Result<String, &'static str> {
    let mut string = String::new();

    loop {
        advance(contents_chars, current_character, current_position);
        if let Some(character) = current_character {  // TODO Temporary
            print!("{}", character);
        }

        if let Some(character) = current_character {
            if *character == '"' {
                break;
            } else {
                string.push(*character);
            }
        } else {  // character is None
            return Err("Missing right double quotes");
        }
    }

    Ok(string)
}

fn build_number(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                    current_position: &mut i32) -> Result<String, &'static str> {
    let mut number = String::new();
    let mut is_floating_point = false;

    let mut last_character = '\0';

    number.push(current_character.unwrap());

    loop {
        advance(contents_chars, current_character, current_position);
        if let Some(character) = current_character {  // TODO Temporary
            print!("{}", character);
        }

        if let Some(character) = current_character {
            match character {
                '0' | '1' |
                '2' | '3' |
                '4' | '5' |
                '6' | '7' |
                '8' | '9' => number.push(*character),

                '.' => {
                    if !is_floating_point {
                        number.push(*character);
                        is_floating_point = true;
                    } else {
                        return Err("Invalid number format");
                    }
                }

                _ => {
                    if last_character == '.' {
                        return Err("Invalid number format");
                    }
                    break;
                }
            }
            last_character = *character;
        } else {  // character is None
            return Err("Reached EOF");
        }
    }

    Ok(number)
}

fn build_keyword(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                 current_position: &mut i32) -> Result<String, &'static str> {
    let mut keyword = String::new();

    keyword.push(current_character.unwrap());

    loop {
        advance(contents_chars, current_character, current_position);
        if let Some(character) = current_character {  // TODO Temporary
            print!("{}", character);
        }

        if let Some(character) = current_character {
            match character {
                'r' | 'u' | 'e' |
                'a' | 'l' | 's' => keyword.push(*character),

                _ => {
                    if !(keyword == "true" || keyword == "false" || keyword == "null") {
                        return Err("Invalid keyword");
                    }
                    break;
                }
            }
        } else {  // character is None
            return Err("Reached EOF");
        }
    }

    Ok(keyword)
}

#[derive(Debug)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(String),  // Integer or float
    Keyword(String),  // Boolean or null
    Unidentified(String)  // When this is used, there will be an error
}

#[derive(Debug)]
pub struct ParseError(pub &'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}
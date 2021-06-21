use std::error::Error;
use std::fmt;

pub fn tokenize(contents: String) -> Result<Vec<Token>, ParseError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_character: Option<char> = None;
    let mut current_position = Position { index: -1, line: 1, column: -1 };

    let contents_chars = contents.chars().collect();

    advance(&contents_chars, &mut current_character, &mut current_position);

    while let Some(character) = current_character {
        // print!("{}", character);
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
           current_position: &mut Position) {
    current_position.advance(current_character);
    if current_position.index < contents_chars.len() as i32 {
        *current_character = Some(contents_chars[current_position.index as usize]);
    } else {
        *current_character = None;
    }
}

fn build_string(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                current_position: &mut Position) -> Result<String, ParseError> {
    let mut string = String::new();
    // TODO error on invalid char: "\"
    // TODO error (unexpected end of string)
    loop {
        advance(contents_chars, current_character, current_position);
        // if let Some(character) = current_character {  // TODO Temporary
        //     print!("{}", character);
        // }

        if let Some(character) = current_character {
            if *character == '"' {
                break;
            } else {
                string.push(*character);
            }
        } else {  // character is None
            return Err(ParseError::new("Missing right double quotes",
                       current_position.line, current_position.column));
        }
    }

    Ok(string)
}

fn build_number(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                    current_position: &mut Position) -> Result<String, ParseError> {
    let mut number = String::new();
    let mut is_floating_point = false;

    let mut last_character = '\0';

    number.push(current_character.unwrap());

    loop {
        advance(contents_chars, current_character, current_position);
        // if let Some(character) = current_character {  // TODO Temporary
        //     print!("{}", character);
        // }

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
                        // return Err("Invalid number format");
                        return Err(ParseError::new("Invalid number format", current_position.line,
                                       current_position.column));
                    }
                }

                _ => {
                    if last_character == '.' {
                        return Err(ParseError::new("Invalid number format", current_position.line,
                                       current_position.column));
                    }
                    break;
                }
            }
            last_character = *character;
        } else {  // character is None
            return Err(ParseError::new("Reached EOF", current_position.line,
                                       current_position.column));
        }
    }

    Ok(number)
}

fn build_keyword(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                 current_position: &mut Position) -> Result<String, ParseError> {
    let mut keyword = String::new();

    keyword.push(current_character.unwrap());

    loop {
        advance(contents_chars, current_character, current_position);
        // if let Some(character) = current_character {  // TODO Temporary
        //     print!("{}", character);
        // }

        if let Some(character) = current_character {
            match character {
                'r' | 'u' | 'e' |
                'a' | 'l' | 's' => keyword.push(*character),

                _ => {
                    if !(keyword == "true" || keyword == "false" || keyword == "null") {
                        return Err(ParseError::new("Invalid keyword", current_position.line,
                                       current_position.column));
                    }
                    break;
                }
            }
        } else {  // character is None
            return Err(ParseError::new("Reached EOF", current_position.line,
                                       current_position.column));
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
pub struct ParseError {
    message: &'static str,
    line: i32,
    column: i32
}

impl ParseError {
    fn new(message: &'static str, line: i32, column: i32) -> Self {
        Self {
            message: message,
            line: line,
            column: column
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Parse error: {}\nLine: {}, column: {}", self.message,
               self.line, self.column)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone)]
struct Position {
    pub index: i32,  // Character index in JSON file
    pub line: i32,
    pub column: i32
}

impl Position {
    fn advance(&mut self, current_character: &Option<char>) -> &mut Self {
        self.index += 1;
        self.column += 1;

        let character = match current_character {
            Some(character) => *character,
            None => '\0'
        };

        if character == '\n' || character == '\r' {
            self.line += 1;
            self.column = 0;
        }

        self
    }
}

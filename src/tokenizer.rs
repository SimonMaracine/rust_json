use std::error::Error;
use std::fmt;

pub(crate) fn tokenize(contents: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_character: Option<char> = None;
    let mut current_position = Position { index: -1, line: 1, column: -1 };

    let contents_chars = contents.chars().collect();

    advance(&contents_chars, &mut current_character, &mut current_position);

    while let Some(character) = current_character {
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
                    Err(error) => return Err(error)
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
                    Err(error) => return Err(error)
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
                    Err(error) => return Err(error)
                }

                // This is so that the character after the last keyword
                // character is evaluated (to not call advance())
                continue;
            }

            _ => return Err(Box::new(
                    ParseError::new(format!("Unidentified character: {}", character),
                                    current_position.line,
                                    current_position.column))
                 )
        }
        advance(&contents_chars, &mut current_character, &mut current_position);
    }

    tokens.push(Token::Eof);

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
                current_position: &mut Position) -> Result<String, Box<dyn Error>> {
    let mut string = String::new();

    let mut last_character = '\0';
    let mut check_escape_character = false;

    loop {
        advance(contents_chars, current_character, current_position);

        if let Some(character) = current_character {
            if check_escape_character {
                match *character {
                    '"' => string.push('\"'),
                    '\\' => string.push('\\'),
                    'n' => string.push('\n'),
                    'r' => string.push('\r'),
                    't' => string.push('\t'),
                    _ => return Err(Box::new(
                        InvalidStringError::new("Unknown escape character in string",
                                                current_position.line,
                                                current_position.column))
                    )
                }
                check_escape_character = false;
            } else {
                match *character {
                    '"' => break,
                    '\\' => check_escape_character = true,
                    '\n' => return Err(Box::new(
                        InvalidStringError::new("Unexpected end of string",
                                                current_position.line,
                                                current_position.column)
                    )),
                    _ => string.push(*character)
                }
            }
        } else {  // character is None
            return Err(Box::new(InvalidStringError::new("Missing right double quotes",
                                                        current_position.line,
                                                        current_position.column)));
        }
    }

    Ok(string)
}

fn build_number(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                current_position: &mut Position) -> Result<String, Box<dyn Error>> {
    let mut number = String::new();
    let mut is_floating_point = false;

    let mut last_character = '\0';

    number.push(current_character.unwrap());

    loop {
        advance(contents_chars, current_character, current_position);

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
                        return Err(Box::new(
                            ParseError::new("Invalid number format".to_string(),
                                            current_position.line,
                                            current_position.column))
                        );
                    }
                }

                _ => {
                    if last_character == '.' {
                        return Err(Box::new(
                            ParseError::new("Invalid number format".to_string(),
                                            current_position.line,
                                            current_position.column))
                        );
                    }
                    break;
                }
            }
            last_character = *character;
        } else {  // character is None
            return Err(Box::new(EofError::new("Reached EOF", current_position.line,
                                              current_position.column)));
        }
    }

    Ok(number)
}

fn build_keyword(contents_chars: &Vec<char>, current_character: &mut Option<char>,
                 current_position: &mut Position) -> Result<String, Box<dyn Error>> {
    let mut keyword = String::new();

    keyword.push(current_character.unwrap());

    loop {
        advance(contents_chars, current_character, current_position);

        if let Some(character) = current_character {
            match character {
                'r' | 'u' | 'e' |
                'a' | 'l' | 's' => keyword.push(*character),

                _ => {
                    if !(keyword == "true" || keyword == "false" || keyword == "null") {
                        return Err(Box::new(ParseError::new("Invalid keyword".to_string(),
                                                            current_position.line,
                                                            current_position.column)));
                    }
                    break;
                }
            }
        } else {  // character is None
            return Err(Box::new(EofError::new("Reached EOF", current_position.line,
                                              current_position.column)));
        }
    }

    Ok(keyword)
}

#[derive(Debug)]
pub(crate) enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    Eof,
    String(String),
    Number(String),  // Integer or float
    Keyword(String),  // Boolean or null
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
    line: i32,
    column: i32
}

impl ParseError {
    fn new(message: String, line: i32, column: i32) -> Self {
        Self {
            message: message,
            line: line,
            column: column
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "ParseError: {}\nLine: {}, column: {}", self.message,
               self.line, self.column)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct InvalidStringError {
    message: &'static str,
    line: i32,
    column: i32
}

impl InvalidStringError {
    fn new(message: &'static str, line: i32, column: i32) -> Self {
        Self {
            message: message,
            line: line,
            column: column
        }
    }
}

impl fmt::Display for InvalidStringError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "InvalidStringError: {}\nLine: {}, column: {}", self.message,
               self.line, self.column)
    }
}

impl Error for InvalidStringError {}

#[derive(Debug)]
pub struct EofError {
    message: &'static str,
    line: i32,
    column: i32
}

impl EofError {
    fn new(message: &'static str, line: i32, column: i32) -> Self {
        Self {
            message: message,
            line: line,
            column: column
        }
    }
}

impl fmt::Display for EofError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "EOFError: {}\nLine: {}, column: {}", self.message,
               self.line, self.column)
    }
}

impl Error for EofError {}

#[derive(Debug, Clone)]
struct Position {
    index: i32,  // Character index in JSON file
    line: i32,
    column: i32
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

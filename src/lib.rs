#![allow(unused)]

mod data_structure;

use std::fs::read_to_string;
use std::error::Error;
use std::fmt;
use data_structure::{JsonObject, JsonArray, ArrayType, ArrayTypeRef};

pub fn load<'object>(file: String) -> Result<JsonObject<'object>, Box<dyn Error>> {
    let contents = read_to_string(file)?;
    let tokens = tokenize(contents);

    match tokens {
        Ok(token_arr) => println!("{:#?}", token_arr),
        Err(message) => return Err(Box::new(ParseError(message)))
    }

    Ok(JsonObject::new())
}

#[allow(unused)]
pub fn dump(object: JsonObject) -> String {
    String::new()
}

fn tokenize(contents: String) -> Result<Vec<Token>, &'static str> {
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
enum Token {
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
struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let result = load(String::from("samples/sample1.json"));
        if let Err(error) = result {
            println!("{}", error);
        }
    }

    #[test]
    fn data_structure() {
        let mut object = JsonObject::new();
        object.insert_int("Simon", 18);
        object.insert_bool("male", true);

        assert!(object.get_object("Foo").is_none());
        assert_eq!(object.get_int("Simon").expect("Is none"), 18);
        // object.get_int("some_key");

        let mut array = JsonArray::new();
        array.add_int(18);
        array.add_int(19);
        array.add_float(18.6);
        array.add_int(20);
        array.add_int(21);
        array.add_string(String::from("Simon"));
        array.add_int(22);
        // println!("{:#?}", array);

        let num = array.get(0);
        if let Ok(value) = num {
            if let ArrayTypeRef::Int(val) = value {
                println!("{}", val);
            }
        }

        match array.remove(0) {
            Ok(value) => {
                if let ArrayType::Int(val) = value {
                    println!("{}", val);
                }
            }
            Err(e) => println!("{}", e)
        }
        match array.remove(1) {
            Ok(value) => println!("{:?}", value),
            Err(e) => println!("{}", e)
        }
        match array.remove(3) {
            Ok(value) => println!("{:?}", value),
            Err(e) => println!("{}", e)
        }
        // println!("{:#?}", array);
    }

    #[test]
    fn real_life_example() {
        let mut base = JsonObject::new();
        let mut new_base = base.clone();

        let mut my_array = JsonArray::new();
        my_array.add_int(12);
        my_array.add_bool(true);
        my_array.add_string(String::from("Simon"));

        base.insert_float("my_float", 18.95);
        base.insert_array("my_array", my_array);

        // Get the array, mutate it and put it back
        let mut result = base.get_array("my_array");
        if let Some(mut array) = result {
            array.add_int(2);
            array.add_float(3.1415);

            println!("{:#?}", base);

            new_base.insert_array("my_array", array);

            println!("{:#?}", new_base);
        }
    }
}

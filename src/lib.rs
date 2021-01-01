mod data_structure;

use std::fs::read_to_string;
use std::error::Error;
use data_structure::JSONObject;

pub fn load(file: String) -> Result<JSONObject, Box<dyn Error>> {
    let contents = read_to_string(file)?;
    let tokens = tokenize(&contents);
    println!("{:#?}", tokens);



    Ok(JSONObject::new())
}

pub fn dump(json: JSONObject) -> String {
    String::new()
}



fn tokenize(contents: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut state = State::InNothing;

    let mut string = String::new();
    let mut number = String::new();
    let mut keyword = String::new();

    let mut capture_keyword_token = false;

    let mut last_character: char = '\0';

    for character in contents.chars() {
        print!("{} ", character);

        match character {
            '{' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                }

                if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::LeftBrace);
                }
            }
            '[' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                }

                if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::LeftBracket);
                }
            }
            '}' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                }

                if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::RightBrace);
                }
            }
            ']' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                }

                if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::RightBracket);
                }
            }
            ':' => {
                if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::Colon);
                }
            }
            ',' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                }

                if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::Comma);
                }
            }
            '"' => {  // TODO this can be in string, if there is a \ before
                if let State::InString = state  {
                    if last_character != '\\' {
                        tokens.push(Token::String(string.clone()));
                        string.clear();
                        state = State::InNothing;  // Not really in nothing, but not in string
                    } else {
                        string.push(character);
                    }
                } else {
                    state = State::InString;
                }
            }
            '0' | '1' | '2' | '3' |
            '4' | '5' | '6' | '7' |
            '8' | '9' => {
                if let State::InNumber = state {
                    number.push(character);
                } else {
                    state = State::InNumber;
                    number.push(character);
                }
            }
            '\\' => {
                if let State::InString = state {
                    if last_character == '\\' {
                        string.push(character);
                    }
                }
            }
            ' ' | '\n' |
            '\r' | '\t' => {
                if let State::InString = state {
                    string.push(character);
                }  // Else ignore it completely
            }
            _ => {  // Letters and other characters
                if let State::InString = state {
                    string.push(character);
                } else if capture_keyword_token {
                    keyword.push(character);
                }


            }
        }

        last_character = character;
    }

    tokens
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
    Keyword(String)  // Boolean or null
}

enum State {
    InNothing,
    InObject,
    InArray,
    InString,
    InNumber,
    InBool,
    InNull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        load(String::from("samples/sample1.json"));
        println!("{:?}", Token::String(String::from("Si\"mon")));
        // println!("####################################");
        // load(String::from("samples/empty.json"));
    }

    #[test]
    fn data_structure() {
        let mut object = JSONObject::new();
        object.insert_int(String::from("Simon"), 18);
        object.insert_bool(String::from("male"), true);

        assert!(object.get_object(String::from("Foo")).is_none());
        assert_eq!(*object.get_int(String::from("Simon")).expect("Is none"), 18);
    }
}

// match character {
//     '{' => {
//         state_stack.push(State::InObject);
//     },

//     '[' => {
//         state_stack.push(State::InArray);
//     },
//     '"' => {
//         let state = match state_stack.last() {
//             Some(s) => s,
//             None => &State::InNothing
//         };

//         if let State::InString = *state  {
//             state_stack.push(State::InString);
//         } else {
//             state_stack.pop();
//         }
//     },
//     ']' | '}' => {
//         state_stack.pop();
//     },
//     _ => ()
// }

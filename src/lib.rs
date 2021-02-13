mod data_structure;

use std::fs::read_to_string;
use std::error::Error;
use data_structure::{JSONObject, JSONArray, ArrayType, ArrayTypeRef};

pub fn load(file: String) -> Result<JSONObject, Box<dyn Error>> {
    let contents = read_to_string(file)?;
    let tokens = tokenize(contents);
    println!("{:#?}", tokens);



    Ok(JSONObject::new())
}

pub fn dump(json: JSONObject) -> String {
    String::new()
}

fn tokenize(contents: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut state = State::InNothing;

    let mut string = String::new();
    let mut number = String::new();
    let mut keyword = String::new();

    let mut last_character = '\0';

    for character in contents.chars() {
        print!("{} ", character);

        match character {
            '{' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                } else if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::LeftBrace);  // May be an error
                }
            }
            '[' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                } else if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::LeftBracket);  // May be an error
                }
            }
            '}' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                } else if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::RightBrace);  // May be an error
                }
            }
            ']' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                } else if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::RightBracket);  // May be an error
                }
            }
            ':' => {
                if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::Colon);  // May be an error
                }
            }
            ',' => {
                if let State::InNumber = state {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                    state = State::InNothing;  // Not really in nothing, but not in number
                } else if let State::InString = state {
                    string.push(character);
                } else {
                    tokens.push(Token::Comma);  // May be an error
                }
            }
            '"' => {
                if let State::InString = state  {
                    if last_character != '\\' {
                        tokens.push(Token::String(string.clone()));
                        string.clear();
                        state = State::InNothing;  // Not really in nothing, but not in string
                    } else {
                        string.push(character);
                    }
                } else if last_character == ':' || last_character == '[' || last_character == ',' || last_character == '{' {
                    state = State::InString;
                } else {
                    tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                }
            }
            '.' => {
                if let State::InString = state  {
                    string.push(character);
                } else if let State::InNumber = state {
                    number.push(character);
                } else {
                    tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                }
            }
            '0' | '1' | '2' | '3' |
            '4' | '5' | '6' | '7' |
            '8' | '9' => {
                if let State::InString = state {
                    string.push(character);
                } else if let State::InNumber = state {
                    number.push(character);
                } else if last_character == ':' || last_character == '[' || last_character == ',' {
                    number.push(character);
                    state = State::InNumber;
                } else {
                    tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                }
            }
            '\\' => {
                if let State::InString = state {
                    if last_character == '\\' {
                        string.push(character);
                    } else {
                        tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                    }
                }
            }
            ' ' | '\n' |
            '\r' | '\t' => {
                if let State::InString = state {
                    if character != '\n' {
                        string.push(character);
                    } else {
                        tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                    }
                }
                // Else ignore it completely
            }
            't' | 'f' | 'n' => {
                if let State::InString = state {
                    string.push(character);
                } else if last_character == ':' || last_character == '[' || last_character == ',' {
                    keyword.push(character);
                    state = State::InKeyword;
                } else {
                    tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                }
            }
            'e' | 'l' => {
                if let State::InString = state {
                    string.push(character);
                } else if let State::InKeyword = state {
                    if character == 'e' {
                        keyword.push(character);
                        tokens.push(Token::Keyword(keyword.clone()));
                        keyword.clear();
                        state = State::InNothing;
                    } else {  // Must be 'l'
                        if last_character == 'l' {
                            keyword.push(character);
                            tokens.push(Token::Keyword(keyword.clone()));
                            keyword.clear();
                            state = State::InNothing;
                        } else {
                            keyword.push(character);
                        }
                    }
                } else {
                    tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                }
            }
            _ => {  // Other letters and characters
                if let State::InString = state {
                    string.push(character);
                } else if let State::InKeyword = state {
                    keyword.push(character);
                } else {
                    tokens.push(Token::Unidentified(character.to_string()));  // Will be error
                }
            }
        }

        match character {  // Don't keep whitespace
            ' ' | '\n' |
            '\r' | '\t' => (),
            _ => last_character = character
        }
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
    Keyword(String),  // Boolean or null
    Unidentified(String)  // When this is used, there will be an error
}

enum State {
    InNothing,
    InString,
    InNumber,
    InKeyword
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        load(String::from("samples/sample1.json"));
        // println!("####################################");
        // load(String::from("samples/empty.json"));
        // println!("####################################");
        // load(String::from("samples/sample2.json"));
        // println!("####################################");
        // load(String::from("samples/errors.json"));
        // println!("####################################");
        // load(String::from("samples/sample3.json"));
    }

    #[test]
    fn data_structure() {
        let mut object = JSONObject::new();
        object.insert_int(String::from("Simon"), 18);
        object.insert_bool(String::from("male"), true);

        assert!(object.get_object(String::from("Foo")).is_none());
        assert_eq!(*object.get_int(String::from("Simon")).expect("Is none"), 18);

        let mut array: JSONArray = JSONArray::new();
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

    // struct Foo {
    //     x: i32,
    //     y: i32
    // }

    // struct Bar {
    //     arr: Vec<Foo>
    // }

    // impl Bar {
    //     fn get(&mut self, index: usize) -> Option<&mut Foo> {
    //         for (i, item) in self.arr.iter_mut().enumerate() {
    //             if i == index {
    //                 return Some(item);
    //             }
    //         }
    //         None
    //     }
    // }

    #[test]
    fn test_vec() {
        // let vec = vec![
        //     Foo { x: 1, y: 2},
        //     Foo { x: 3, y: 4}
        // ];

        // let mut bar = Bar { arr: vec };

        // let foo = bar.get(0);
        // if let Some(value) = foo {
        //     value.x = 18;
        // }

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

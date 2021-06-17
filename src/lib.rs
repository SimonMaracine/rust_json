#![allow(unused)]

mod data_structure;
mod tokenizer;

use std::fs::read_to_string;
use std::error::Error;

use data_structure::{JsonObject, JsonArray, ArrayType, ArrayTypeRef};
use tokenizer::{tokenize, ParseError};

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

#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct JsonObject<'a> {
    int_pairs: HashMap<&'a str, i32>,
    float_pairs: HashMap<&'a str, f32>,
    bool_pairs: HashMap<&'a str, bool>,
    string_pairs: HashMap<&'a str, &'a String>,
    array_pairs: HashMap<&'a str, &'a JsonArray<'a>>,
    object_pairs: HashMap<&'a str, &'a JsonObject<'a>>,
    null_pairs: HashMap<&'a str, Null>
}

impl<'a> JsonObject<'a> {
    pub fn new() -> Self {
        JsonObject {
            int_pairs: HashMap::new(),
            float_pairs: HashMap::new(),
            bool_pairs: HashMap::new(),
            string_pairs: HashMap::new(),
            array_pairs: HashMap::new(),
            object_pairs: HashMap::new(),
            null_pairs: HashMap::new()
        }
    }

    pub fn insert_int(&mut self, key: &'a str, value: i32) {
        self.int_pairs.insert(key, value);
    }

    pub fn insert_float(&mut self, key: &'a str, value: f32) {
        self.float_pairs.insert(key, value);
    }

    pub fn insert_bool(&mut self, key: &'a str, value: bool) {
        self.bool_pairs.insert(key, value);
    }

    pub fn insert_string(&mut self, key: &'a str, value: &'a String) {
        self.string_pairs.insert(key, value);
    }

    pub fn insert_array(&mut self, key: &'a str, value: &'a JsonArray) {
        self.array_pairs.insert(key, value);
    }

    pub fn insert_object(&mut self, key: &'a str, value: &'a JsonObject<'a>) {
        self.object_pairs.insert(key, value);
    }

    pub fn insert_null(&mut self, key: &'a str, value: Null) {
        self.null_pairs.insert(key, value);
    }

    pub fn delete_int(&mut self, key: &'a str) -> Option<i32> {
        self.int_pairs.remove(&key)
    }

    pub fn delete_float(&mut self, key: &'a str) -> Option<f32> {
        self.float_pairs.remove(&key)
    }

    pub fn delete_bool(&mut self, key: &'a str) -> Option<bool> {
        self.bool_pairs.remove(key)
    }

    pub fn delete_string(&mut self, key: &'a str) -> Option<&String> {
        self.string_pairs.remove(key)
    }

    pub fn delete_array(&mut self, key: &'a str) -> Option<&JsonArray> {
        self.array_pairs.remove(key)
    }

    pub fn delete_object(&mut self, key: &'a str) -> Option<&JsonObject> {
        self.object_pairs.remove(key)
    }

    pub fn delete_null(&mut self, key: &'a str) -> Option<Null> {
        self.null_pairs.remove(key)
    }

    pub fn get_int(&self, key: &'a str) -> Option<&i32> {
        self.int_pairs.get(key)
    }

    pub fn get_float(&self, key: &'a str) -> Option<&f32> {
        self.float_pairs.get(key)
    }

    pub fn get_bool(&self, key: &'a str) -> Option<&bool> {
        self.bool_pairs.get(key)
    }

    pub fn get_string(&self, key: &'a str) -> Option<&&String> {
        self.string_pairs.get(key)
    }

    pub fn get_array(&self, key: &'a str) -> Option<&&JsonArray> {
        self.array_pairs.get(key)
    }

    pub fn get_object(&self, key: &'a str) -> Option<&&JsonObject> {
        self.object_pairs.get(key)
    }

    pub fn get_null(&self, key: &'a str) -> Option<&Null> {
        self.null_pairs.get(key)
    }
}

#[derive(Debug)]
pub struct JsonArray<'a> {
    ints: Vec<ArrayItem<i32>>,
    floats: Vec<ArrayItem<f32>>,
    bools: Vec<ArrayItem<bool>>,
    strings: Vec<ArrayItemRef<'a, String>>,
    arrays: Vec<ArrayItemRef<'a, JsonArray<'a>>>,
    objects: Vec<ArrayItemRef<'a, JsonObject<'a>>>,
    nulls: Vec<ArrayItem<Null>>,

    item_count: usize
}

impl<'a> JsonArray<'a> {
    pub fn new() -> Self {
        JsonArray {
            ints: Vec::new(),
            floats: Vec::new(),
            bools: Vec::new(),
            strings: Vec::new(),
            arrays: Vec::new(),
            objects: Vec::new(),
            nulls: Vec::new(),
            item_count: 0
        }
    }

    pub fn add_int(&mut self, value: i32) {
        self.ints.push(
            ArrayItem {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_float(&mut self, value: f32) {
        self.floats.push(
            ArrayItem {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_bool(&mut self, value: bool) {
        self.bools.push(
            ArrayItem {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_string(&mut self, value: &'a String) {
        self.strings.push(
            ArrayItemRef {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_array(&mut self, value: &'a JsonArray) {
        self.arrays.push(
            ArrayItemRef {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_object(&mut self, value: &'a JsonObject) {
        self.objects.push(
            ArrayItemRef {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_null(&mut self, value: Null) {
        self.nulls.push(
            ArrayItem {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn remove(&mut self, index: usize) -> Result<ArrayType, &str> {
        let mut index_to_remove: isize = -1;
        let mut array_type = ArrayItemType::Int;

        for (i, item) in self.ints.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
                array_type = ArrayItemType::Int;
                break;
            }
        }
        for (i, item) in self.floats.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
                array_type = ArrayItemType::Float;
                break;
            }
        }
        for (i, item) in self.bools.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
                array_type = ArrayItemType::Bool;
                break;
            }
        }
        for (i, item) in self.strings.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
                array_type = ArrayItemType::String_;
                break;
            }
        }
        for (i, item) in self.arrays.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
                array_type = ArrayItemType::Array;
                break;
            }
        }
        for (i, item) in self.objects.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
                array_type = ArrayItemType::Object;
                break;
            }
        }
        for (i, item) in self.nulls.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
                array_type = ArrayItemType::Null_;
                break;
            }
        }

        if index_to_remove == -1 {
            return Err("Index doesn't exist");
        }

        match array_type {
            ArrayItemType::Int => {
                let result = self.ints.remove(index_to_remove as usize);
                self.fix_index_on_array_item_deletion(index);
                self.item_count -= 1;
                Ok(ArrayType::Int(result.item))
            }
            ArrayItemType::Float => {
                let result = self.floats.remove(index_to_remove as usize);
                self.fix_index_on_array_item_deletion(index);
                self.item_count -= 1;
                Ok(ArrayType::Float(result.item))
            }
            ArrayItemType::Bool => {
                let result = self.bools.remove(index_to_remove as usize);
                self.fix_index_on_array_item_deletion(index);
                self.item_count -= 1;
                Ok(ArrayType::Bool(result.item))
            }
            ArrayItemType::String_ => {
                let result = self.strings.remove(index_to_remove as usize);
                self.fix_index_on_array_item_deletion(index);
                self.item_count -= 1;
                Ok(ArrayType::String_(result.item))
            }
            ArrayItemType::Array => {
                let result = self.arrays.remove(index_to_remove as usize);
                self.fix_index_on_array_item_deletion(index);
                self.item_count -= 1;
                Ok(ArrayType::Array(result.item))
            }
            ArrayItemType::Object => {
                let result = self.objects.remove(index_to_remove as usize);
                self.fix_index_on_array_item_deletion(index);
                self.item_count -= 1;
                Ok(ArrayType::Object(result.item))
            }
            ArrayItemType::Null_ => {
                let result = self.nulls.remove(index_to_remove as usize);
                self.fix_index_on_array_item_deletion(index);
                self.item_count -= 1;
                Ok(ArrayType::Null_(result.item))
            }
        }
    }

    pub fn get(&mut self, index: usize) -> Result<ArrayTypeMut, &str> {
        for (_, item) in self.ints.iter_mut().enumerate() {
            if item.index == index {
                return Ok(ArrayTypeMut::Int(item.item));
            }
        }
        for (_, item) in self.floats.iter_mut().enumerate() {
            if item.index == index {
                return Ok(ArrayTypeMut::Float(item.item));
            }
        }
        for (_, item) in self.bools.iter_mut().enumerate() {
            if item.index == index {
                return Ok(ArrayTypeMut::Bool(item.item));
            }
        }
        for (_, item) in self.strings.iter_mut().enumerate() {
            if item.index == index {
                return Ok(ArrayTypeMut::String_(&mut item.item));
            }
        }
        for (_, item) in self.arrays.iter_mut().enumerate() {
            if item.index == index {
                return Ok(ArrayTypeMut::Array(&mut item.item));
            }
        }
        for (_, item) in self.objects.iter_mut().enumerate() {
            if item.index == index {
                return Ok(ArrayTypeMut::Object(&mut item.item));
            }
        }
        for (_, item) in self.nulls.iter_mut().enumerate() {
            if item.index == index {
                return Ok(ArrayTypeMut::Null_(item.item));
            }
        }

        Err("Index doesn't exist")
    }

    pub fn set_int(&mut self, value: i32, index: usize) -> Result<(), &str> {
        self.ints.push(
            ArrayItem {
                item: value,
                index: index
            }
        );

        for (i, item) in self.ints.iter().enumerate() {  // Not well thought yet
            if item.index == index {
                self.ints.remove(i);
                break;
            }
        }
        for (i, item) in self.floats.iter().enumerate() {
            if item.index == index {
                self.floats.remove(i);
                break;
            }
        }
        for (i, item) in self.bools.iter().enumerate() {
            if item.index == index {
                self.bools.remove(i);
                break;
            }
        }
        for (i, item) in self.strings.iter().enumerate() {
            if item.index == index {
                self.strings.remove(i);
                break;
            }
        }
        for (i, item) in self.arrays.iter().enumerate() {
            if item.index == index {
                self.arrays.remove(i);
                break;
            }
        }
        for (i, item) in self.objects.iter().enumerate() {
            if item.index == index {
                self.objects.remove(i);
                break;
            }
        }
        for (i, item) in self.nulls.iter().enumerate() {
            if item.index == index {
                self.nulls.remove(i);
                break;
            }
        }



        Err("Placeholder")
    }

    pub fn set_float(&mut self, value: f32, index: usize) -> Result<(), &'static str> {
        Err("Placeholder")
    }

    pub fn set_bool(&mut self, value: bool, index: usize) -> Result<(), &'static str> {
        Err("Placeholder")
    }

    pub fn set_string(&mut self, value: String, index: usize) -> Result<(), &'static str> {
        Err("Placeholder")
    }

    pub fn set_array(&mut self, value: JsonArray, index: usize) -> Result<(), &'static str> {
        Err("Placeholder")
    }

    pub fn set_object(&mut self, value: JsonObject, index: usize) -> Result<(), &'static str> {
        Err("Placeholder")
    }

    pub fn set_null(&mut self, value: Null, index: usize) -> Result<(), &'static str> {
        Err("Placeholder")
    }

    fn fix_index_on_array_item_deletion(&mut self, index: usize) {
        if index == self.item_count - 1 {
            return;
        }

        let mut index_to_fix = index + 1;

        for _ in 0..self.item_count - index_to_fix {
            for item in self.ints.iter_mut() {
                if item.index == index_to_fix {
                    item.index -= 1;
                    index_to_fix += 1;
                    assert!(index_to_fix != self.item_count + 1);
                    continue;
                }
            }
            for item in self.floats.iter_mut() {
                if item.index == index_to_fix {
                    item.index -= 1;
                    index_to_fix += 1;
                    assert!(index_to_fix != self.item_count + 1);
                    continue;
                }
            }
            for item in self.bools.iter_mut() {
                if item.index == index_to_fix {
                    item.index -= 1;
                    index_to_fix += 1;
                    assert!(index_to_fix != self.item_count + 1);
                    continue;
                }
            }
            for item in self.strings.iter_mut() {
                if item.index == index_to_fix {
                    item.index -= 1;
                    index_to_fix += 1;
                    assert!(index_to_fix != self.item_count + 1);
                    continue;
                }
            }
            for item in self.objects.iter_mut() {
                if item.index == index_to_fix {
                    item.index -= 1;
                    index_to_fix += 1;
                    assert!(index_to_fix != self.item_count + 1);
                    continue;
                }
            }
            for item in self.nulls.iter_mut() {
                if item.index == index_to_fix {
                    item.index -= 1;
                    index_to_fix += 1;
                    assert!(index_to_fix != self.item_count + 1);
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
struct ArrayItem<T> {
    item: T,
    index: usize
}

#[derive(Debug)]
struct ArrayItemRef<'a, T> {
    item: &'a T,
    index: usize
}

enum ArrayItemType {
    Int,
    Float,
    Bool,
    String_,
    Array,
    Object,
    Null_
}

#[derive(Debug)]
pub enum ArrayType<'a, 'b> {
    Int(i32),
    Float(f32),
    Bool(bool),
    String_(&'a String),
    Array(&'a JsonArray<'b>),
    Object(&'a JsonObject<'b>),
    Null_(Null)
}

#[derive(Debug)]
pub enum ArrayTypeMut<'a, 'b> {
    Int(i32),
    Float(f32),
    Bool(bool),
    String_(&'a mut String),
    Array(&'a mut JsonArray<'b>),
    Object(&'a mut JsonObject<'b>),
    Null_(Null)
}

// The null value in JSON
#[derive(Debug, Clone, Copy)]
pub struct Null;



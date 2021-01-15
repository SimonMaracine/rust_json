use std::collections::HashMap;

#[derive(Debug)]
pub struct JSONObject {
    int_pairs: HashMap<String, i32>,
    float_pairs: HashMap<String, f32>,
    bool_pairs: HashMap<String, bool>,
    string_pairs: HashMap<String, String>,
    array_pairs: HashMap<String, JSONArray>,
    object_pairs: HashMap<String, JSONObject>,
    null_pairs: HashMap<String, Null>
}

impl JSONObject {
    pub fn new() -> Self {
        JSONObject {
            int_pairs: HashMap::new(),
            float_pairs: HashMap::new(),
            bool_pairs: HashMap::new(),
            string_pairs: HashMap::new(),
            array_pairs: HashMap::new(),
            object_pairs: HashMap::new(),
            null_pairs: HashMap::new()
        }
    }

    pub fn insert_int(&mut self, key: String, value: i32) {
        self.int_pairs.insert(key, value);
    }

    pub fn insert_float(&mut self, key: String, value: f32) {
        self.float_pairs.insert(key, value);
    }

    pub fn insert_bool(&mut self, key: String, value: bool) {
        self.bool_pairs.insert(key, value);
    }

    pub fn insert_string(&mut self, key: String, value: String) {
        self.string_pairs.insert(key, value);
    }

    pub fn insert_array(&mut self, key: String, value: JSONArray) {
        self.array_pairs.insert(key, value);
    }

    pub fn insert_object(&mut self, key: String, value: JSONObject) {
        self.object_pairs.insert(key, value);
    }

    pub fn insert_null(&mut self, key: String, value: Null) {
        self.null_pairs.insert(key, value);
    }

    pub fn delete_int(&mut self, key: String) -> Option<i32> {
        self.int_pairs.remove(&key)
    }

    pub fn delete_float(&mut self, key: String) -> Option<f32> {
        self.float_pairs.remove(&key)
    }

    pub fn delete_bool(&mut self, key: String) -> Option<bool> {
        self.bool_pairs.remove(&key)
    }

    pub fn delete_string(&mut self, key: String) -> Option<String> {
        self.string_pairs.remove(&key)
    }

    pub fn delete_array(&mut self, key: String) -> Option<JSONArray> {
        self.array_pairs.remove(&key)
    }

    pub fn delete_object(&mut self, key: String) -> Option<JSONObject> {
        self.object_pairs.remove(&key)
    }

    pub fn delete_null(&mut self, key: String) -> Option<Null> {
        self.null_pairs.remove(&key)
    }

    pub fn get_int(&self, key: String) -> Option<&i32> {
        self.int_pairs.get(&key)
    }

    pub fn get_float(&self, key: String) -> Option<&f32> {
        self.float_pairs.get(&key)
    }

    pub fn get_bool(&self, key: String) -> Option<&bool> {
        self.bool_pairs.get(&key)
    }

    pub fn get_string(&self, key: String) -> Option<&String> {
        self.string_pairs.get(&key)
    }

    pub fn get_array(&self, key: String) -> Option<&JSONArray> {
        self.array_pairs.get(&key)
    }

    pub fn get_object(&self, key: String) -> Option<&JSONObject> {
        self.object_pairs.get(&key)
    }

    pub fn get_null(&self, key: String) -> Option<&Null> {
        self.null_pairs.get(&key)
    }
}

#[derive(Debug)]
pub struct JSONArray {
    ints: Vec<ArrayItem<i32>>,
    floats: Vec<ArrayItem<f32>>,
    bools: Vec<ArrayItem<bool>>,
    strings: Vec<ArrayItem<String>>,
    arrays: Vec<ArrayItem<JSONArray>>,
    objects: Vec<ArrayItem<JSONObject>>,
    nulls: Vec<ArrayItem<Null>>,

    item_count: usize
}

impl JSONArray {
    pub fn new() -> Self {
        JSONArray {
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

    pub fn add_string(&mut self, value: String) {
        self.strings.push(
            ArrayItem {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_array(&mut self, value: JSONArray) {
        self.arrays.push(
            ArrayItem {
                item: value,
                index: self.item_count
            }
        );
        self.item_count += 1;
    }

    pub fn add_object(&mut self, value: JSONObject) {
        self.objects.push(
            ArrayItem {
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
        let mut index_to_remove: isize = -1;  // The index of the object in the ints array
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
pub enum ArrayType {
    Int(i32),
    Float(f32),
    Bool(bool),
    String_(String),
    Array(JSONArray),
    Object(JSONObject),
    Null_(Null)
}

// The null value in JSON
#[derive(Debug)]
pub struct Null;

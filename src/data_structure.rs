use std::collections::HashMap;

#[derive(Debug)]
pub struct JSONObject {
    int_pairs: HashMap<String, i32>,
    float_pairs: HashMap<String, f32>,
    bool_pairs: HashMap<String, bool>,
    string_pairs: HashMap<String, String>,
    // array_pairs: HashMap<String, >,
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
            // array_pairs: HashMap::new(),
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

    pub fn get_object(&self, key: String) -> Option<&JSONObject> {
        self.object_pairs.get(&key)
    }

    pub fn get_null(&self, key: String) -> Option<&Null> {
        self.null_pairs.get(&key)
    }
}

#[derive(Debug)]
pub struct Array {
    ints: Vec<ArrayItem<i32>>,
    floats: Vec<ArrayItem<f32>>,
    bools: Vec<ArrayItem<bool>>,
    strings: Vec<ArrayItem<String>>,
    arrays: Vec<ArrayItem<Array>>,
    objects: Vec<ArrayItem<JSONObject>>,
    nulls: Vec<ArrayItem<Null>>,

    item_count: usize
}

impl Array {
    pub fn new() -> Self {
        Array {
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

    pub fn remove_int(&mut self, index: usize) {
        let mut index_to_remove: isize = -1;
        for (i, item) in self.ints.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
            }
        }
        if index_to_remove == -1 {
            panic!("Index {} doesn't exist");
        }

        self.ints.remove(index_to_remove as usize);
    }

    pub fn remove_float(&mut self, index: usize) {
        let mut index_to_remove: isize = -1;
        for (i, item) in self.floats.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
            }
        }
        if index_to_remove == -1 {
            panic!("Index {} doesn't exist");
        }

        self.floats.remove(index_to_remove as usize);
    }

    pub fn remove_bool(&mut self, index: usize) {
        let mut index_to_remove: isize = -1;
        for (i, item) in self.bools.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
            }
        }
        if index_to_remove == -1 {
            panic!("Index {} doesn't exist");
        }

        self.bools.remove(index_to_remove as usize);
    }

    pub fn remove_string(&mut self, index: usize) {
        let mut index_to_remove: isize = -1;
        for (i, item) in self.strings.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
            }
        }
        if index_to_remove == -1 {
            panic!("Index {} doesn't exist");
        }

        self.strings.remove(index_to_remove as usize);
    }

    pub fn remove_object(&mut self, index: usize) {
        let mut index_to_remove: isize = -1;
        for (i, item) in self.objects.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
            }
        }
        if index_to_remove == -1 {
            panic!("Index {} doesn't exist");
        }

        self.objects.remove(index_to_remove as usize);
    }

    pub fn remove_null(&mut self, index: usize) {
        let mut index_to_remove: isize = -1;
        for (i, item) in self.nulls.iter().enumerate() {
            if item.index == index {
                index_to_remove = i as isize;
            }
        }
        if index_to_remove == -1 {
            panic!("Index {} doesn't exist");
        }

        self.nulls.remove(index_to_remove as usize);
    }
}

#[derive(Debug)]
struct ArrayItem<T> {
    item: T,
    index: usize
}

// The null value in json
#[derive(Debug)]
pub struct Null;

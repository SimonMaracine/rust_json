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

// The null value in json
#[derive(Debug)]
pub struct Null;

mod data_structure;

use data_structure::JSONObject;

pub fn load(file: String) -> JSONObject {
    JSONObject::new()  // Temporary
}

pub fn dump(json: JSONObject) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let object = load(String::from("samples/sample1.json"));
        
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

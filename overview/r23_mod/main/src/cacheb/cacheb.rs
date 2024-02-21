pub struct CacheB {}

impl CacheB {
    pub fn new() -> CacheB {
        CacheB {}
    }

    pub fn get(&self, key: String) -> String {
        return "".to_string();
    }

    pub fn set(&self, key: String, value: String) {}
}

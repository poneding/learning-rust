pub struct CacheA {}

impl CacheA {
    pub fn new() -> CacheA {
        CacheA {}
    }

    pub fn get(&self, key: String) -> String {
        return "".to_string();
    }

    pub fn set(&self, key: String, value: String) {}
}

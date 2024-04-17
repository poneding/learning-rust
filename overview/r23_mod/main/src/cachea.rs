use std::collections::HashMap;
pub struct CacheA {
    m: HashMap<String, String>,
}

impl CacheA {
    pub fn new() -> CacheA {
        CacheA { m: HashMap::new() }
    }

    #[allow(unused)]
    pub fn get(&self, key: &str) -> &str {
        match self.m.get(key) {
            Some(v) => v.as_str(),
            None => "",
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.m.insert(key.into(), value.into());
    }
}

use lib::chinese;
use lib::english;

mod cachea;
mod cacheb;

fn main() {
    chinese::hello();
    english::hello();

    let mut ca = cachea::CacheA::new();
    ca.set(String::from("key"), String::from("value"));

    let mut cb = cacheb::cacheb::CacheB::new();
    cb.set(String::from("key"), String::from("value"));
}

use lib::chinese;
use lib::english;

mod cachea;
mod cacheb;

fn main() {
    chinese::hello();
    english::hello();

    let mut ca = cachea::CacheA::new();
    ca.set("k1", "v1");

    let mut cb = cacheb::cacheb::CacheB::new();
    cb.set("k2", "v2");
}

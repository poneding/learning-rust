#[allow(unused)]
use std::collections::HashMap;
use std::rc::Rc;

// Rc: reference counting, not thread safe
// Arc: atomic reference counting, thread safe
fn main() {
    let _rc1 = Rc::new("Hello World!");
    let _rc2 = _rc1.clone(); // not real clone, just increase the reference count
    let _rc3 = _rc1.clone(); // not real clone, just increase the reference count

    assert_eq!(_rc1.contains("Hello"), true);
    assert_eq!(_rc3.contains("World"), true);
}

#[test]
// Shared References
fn _test_demo_ref() {
    type Table = HashMap<String, Vec<String>>;

    // cant read t after show_table_direct called
    fn show_table_direct(t: Table) {
        for (key, value) in t {
            // key: String, value: Vec<String>
            println!("{}:", key);
            for v in value {
                // v: String
                println!("  {}", v);
            }
        }
    }

    fn show_table_ref(t: &Table) {
        for (key, value) in t {
            // key: &String, value: &Vec<String>
            println!("{}:", key);
            for v in value {
                // v: &String
                println!("  {}", v);
            }
        }
    }

    let mut _t = Table::new();
    _t.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    _t.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of Saint Matthew".to_string(),
        ],
    );
    _t.insert(
        "Cellini".to_string(),
        vec!["Perseus with the head of Medusa".to_string()],
    );
    show_table_ref(&_t);

    // mutable reference
    fn sort_works(t: &mut Table) {
        for (_composer, works) in t {
            works.sort();
        }
    }

    sort_works(&mut _t);

    println!("After sort_works:");
    show_table_direct(_t);
}

#[test]
fn _test_demo_ref2() {
    struct Anime1 {
        name: &'static str,
    }

    let _anime1 = Anime1 { name: "Naruto" };
    let _anime1_ref = &_anime1;
    assert_eq!(_anime1_ref.name, "Naruto");
    assert_eq!((*_anime1_ref).name, "Naruto");
}

fn _factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b) // 1 * 2 * 3 * 4 * 5 * 6
}

#[test]
fn _test_factorial() {
    let r = &_factorial(6);
    println!("r:{}", r);
    assert_eq!(r + &1009, 1729);
}

static mut _STASH: &i32 = &128;
fn _staticf(p: &'static i32) {
    unsafe {
        _STASH = p;
    }
}
#[test]
fn _test_staticf() {
    static N1: i32 = 1000;
    _staticf(&N1)
}

fn _g<'a>(_p: &'a i32) {}

#[test]
fn _test_g() {
    let x = 10;
    _g(&x);
}

fn _smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if r < s {
            s = r;
        }
    }
    s
}

#[test]
fn _test_smallest() {
    let arr = [20, 10, 30, 40];
    let r = _smallest(&arr);
    assert_eq!(*r, 10);

    let s;
    {
        let ar2 = [20, 10, 30, 40];
        s = _smallest(&ar2);
        assert_eq!(s, &10); // ok
    }
    // assert_eq!(s, &10);// not ok: ar2 does not live long enough
}

#[allow(dead_code)]
struct StringTable {
    elements: Vec<String>,
}

#[allow(dead_code)]
impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

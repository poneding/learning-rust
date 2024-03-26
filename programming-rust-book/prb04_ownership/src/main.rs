fn main() {
    println!("Hello, world!");
}

#[test]
fn _test_transfer_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("{}", s2);

    struct Persion {
        name: String,
    }

    let composers = vec![
        Persion {
            name: "Palestrina".to_string(),
        },
        Persion {
            name: "Dowland".to_string(),
        },
        Persion {
            name: "Lully".to_string(),
        },
    ];

    // let first_composer_name = composers[0].name;// cannot move out of indexed content
    let first_composer_name = &composers[0].name;
    println!("first_composer_name:{}", first_composer_name);
    println!("&composers[0].name:{}", &composers[0].name);

    struct Persion2 {
        name: Option<String>,
    }
    let mut composers2 = vec![
        Persion2 {
            name: Some("Palestrina".to_string()),
        },
        Persion2 {
            name: Some("Dowland".to_string()),
        },
        Persion2 {
            name: Some("Lully".to_string()),
        },
    ];
    let first_composer_name_v2 = std::mem::replace(&mut composers2[0].name, None);
    assert_eq!(first_composer_name_v2, Some("Palestrina".to_string()));
    assert_eq!(composers2[0].name, None);

    // std::mem::replace == Option::take
    let second_composer_name = composers2[1].name.take();
    assert_eq!(second_composer_name, Some("Dowland".to_string()));
    assert_eq!(composers2[1].name, None);

    // copy trait [int, float, char, bool, or tuple, array with copy field]
    let x = 42;
    let _y = x; // copy x value, and x still can be used

    // struct is not copy trait
    struct Label {
        no: u32,
    }

    let _l = Label { no: 3 };
    fn print_label(l: Label) {
        println!("Show lable no is: {}", l.no)
    }
    // ownership transfered to print_label
    print_label(_l);
    // println!("lable no is: {}", _l.no); // error[E0382]: borrow of moved value: `_l`

    // solution 1: use reference
    fn print_label_v2(l: &Label) {
        println!("Show lable no is: {}", l.no)
    }
    let l = Label { no: 3 };
    print_label_v2(&l);
    println!("lable no is: {}", l.no);

    // solution 2: struct implement Copy trait
    // !!! fileds of struct must implement Copy trait
    #[derive(Copy, Clone)]
    struct Label2 {
        no: u32, // u32 implement Copy trait
    }
    fn print_label2(l: Label2) {
        println!("Show lable no is: {}", l.no)
    }
    let l2 = Label2 { no: 3 };
    print_label2(l2);
    println!("lable no is: {}", l2.no);
}

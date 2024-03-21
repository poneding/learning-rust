extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
// 所有的共有名称暴露
use iron::mime::Mime;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(Mime::from_str("text/html; charset=utf-8").unwrap());
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n" />
            <input type="text" name="n" />
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    ); // 原始字符串语法：r(raw) 跟一个或多个#号，然后是一个双引号，然后是字符串内容，然后是一个双引号，然后是一个或多个#号，即：r#"..."#
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };
    let unparsed_numbers = match form_data.get("n") {
        Some(n) => n,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Ok(n) => {
                numbers.push(n);
            }
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Value for n is not a number: {:?}\n", unparsed));
                return Ok(response);
            }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = _gcd(d, *m);
    }
    response.set_mut(status::Ok);
    response.set_mut(Mime::from_str("text/html; charset=utf-8").unwrap());
    response.set_mut(format!(
        "The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
        numbers, d
    ));
    Ok(response)
}

fn _gcd(mut n: u64, mut m: u64) -> u64 {
    // assert!(n != 0 && m != 0); // assertion failed: n != 0 && m != 0
    assert!(n != 0 && m != 0, "n or m can't be zero");
    print!("The greatest common divisor of {} and {} is ", n, m);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n; // 大数对小数取余，如果余数不为0，继续循环
    }
    print!("{}\n", n);
    n
}

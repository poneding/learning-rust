use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:8080...");
    server
        .bind("127.0.0.1:8080")
        .expect("error binding server to address.")
        .run()
        .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
    <input type="text" name="n" />
    <input type="text" name="m" />
    <button type="submit">Compute GCD</button>
    </form>"#,
    )
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
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

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

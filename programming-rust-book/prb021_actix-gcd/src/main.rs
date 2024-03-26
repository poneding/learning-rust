use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(get_index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="get">
    <input type="text" name="n" />
    <input type="text" name="m" />
    <button type="submit">Compute GCD</button>
    </form>"#,
    )
}

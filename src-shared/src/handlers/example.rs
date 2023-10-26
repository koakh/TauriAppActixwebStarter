use actix_web::get;

#[get("/api/shared/test")]
pub async fn handle_shared_test() -> actix_web::Result<String> {
    let text = "hello world, form shared handlers";
    println!("{}", text);
    Ok(text.to_string())
}
use actix_web::get;
use src_shared::{add, rand};

#[get("/api/test")]
pub async fn handle_test() -> actix_web::Result<String> {
    let text = "hello world";
    println!("{}", text);
    Ok(text.to_string())
}

/// example using shared library
#[get("/api/test-shared-add")]
pub async fn handle_shared_add() -> actix_web::Result<String> {
    let added = add(28, 14);
    println!("{}", added.to_string());
    Ok(added.to_string())
}

/// example using shared library
#[get("/api/test-shared-rand")]
pub async fn handle_rand_rand() -> actix_web::Result<String> {
    let rand = rand();
    println!("{}", rand.to_string());
    Ok(rand.to_string())
}

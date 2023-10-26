mod handlers;

use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};
use tauri::AppHandle;

struct TauriAppState {
    app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            // main handlers
            .service(handlers::example::handle_test)
            .service(handlers::example::handle_shared_add)
            .service(handlers::example::handle_rand_rand)
            // shared handlers
            .service(src_shared::handlers::example::handle_shared_test)
    })
    .bind(("127.0.0.1", 4875))?
    .run()
    .await
}

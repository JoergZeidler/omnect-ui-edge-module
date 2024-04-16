mod api;

use api::omnect_device_service::get_version;

use actix_web::{middleware::Logger, web::scope, App, HttpServer};
use actix_web_lab::web::spa;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(
                scope("/api").service(get_version), // .service(submit_task)
                                                    // .service(start_task)
                                                    // .service(complete_task)
                                                    // .service(pause_task)
                                                    // .service(fail_task),
            )
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}

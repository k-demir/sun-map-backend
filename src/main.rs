use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use env_logger::Env;
use actix_files::Files;

mod coordinates;
mod shadows;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {

        // TODO Do not use `Cors::permissive()`.
        let cors = Cors::permissive();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(coordinates::get_tile_information)
            .service(shadows::get_shadows)
            .service(Files::new("/api/tiles", "./assets/map_images"))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}

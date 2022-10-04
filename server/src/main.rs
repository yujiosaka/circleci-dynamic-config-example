use std::{env, process};

use actix_files as fs;
use actix_web::{App, HttpServer};
use log::{error, info};
use shared::Result;

use server::route;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    if let Err(err) = serve().await {
        error!("{}", err);
        process::exit(1);
    };
}

async fn serve() -> Result<()> {
    let host = env::var("SERVER_HOST")?;
    let port = env::var("SERVER_PORT")?;
    let port = port.parse::<u16>()?;
    info!("Listening on port: {}", port);
    HttpServer::new(|| {
        App::new()
            .service(route::add)
            .service(fs::Files::new("/", "./dist").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await?;
    info!("Stopped listening");
    Ok(())
}

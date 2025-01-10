mod webif;
mod logger;
mod sysinfo;

use std::env;
use actix_multipart::form::MultipartFormConfig;

use crate::webif::fwupdate::{fwupdate_get, fwupdate_upload};
use crate::logger::{ilog, init_logger_custom_format};

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    ilog("Starting up", 3);
    init_logger_custom_format();
    let mut port = 80;
    let listen_on = "0.0.0.0";

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let arg = args[1].clone();
    
        // overwrite settings with config file
        if arg == "-p" {
            port = args[2].parse::<u16>().unwrap_or(80);
            ilog(&format!("Using given port: {:?}", port), 3);
        }
    }

    HttpServer::new(|| {
        App::new()
            .app_data(
            MultipartFormConfig::default()
                .total_limit(120 * 1024 * 1024) // 120 MB
                .memory_limit(30 * 1024 * 1024), // 30 MB
            )
            .wrap(Logger::default())
            .service(
                web::resource(["/fwupdate/doFirmwareUpdate"])
                    .route(web::post().to(fwupdate_upload)),
            )
            .service(web::resource(["/","/fwupdate"]).route(web::get().to(fwupdate_get)))
    })
    .workers(2)
    .bind((listen_on, port))?
    .run()
    .await
    //.map_err(anyhow::Error::from)
}

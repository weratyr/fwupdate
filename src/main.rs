mod webif;
mod logger;
mod sysinfo;
mod statics;

use config::Config;
use std::env;
use actix_multipart::form::MultipartFormConfig;

use crate::webif::fwupdate::{fwupdate_get, fwupdate_upload};
use crate::logger::{ilog, init_logger_custom_format};

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    ilog("Starting up", 3);
    init_logger_custom_format();
    let mut statics = statics::Statics::default();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let arg = args[1].clone();
        let config_file = args[2].clone();
        if arg == "-c" {
            println!("Using config file: {:?}", config_file);
            if sysinfo::file_exists(&config_file) == true {
                let settings = Config::builder()
                    .add_source(config::File::with_name(&config_file))
                    .build()
                    .unwrap();
                statics = statics::Statics::new_from_file(settings.clone());
            } else {
                println!(
                    "Config file found using:
                            DEFAULT Config !!!!!: {:?}\n",
                            config_file
                );
                ilog(
                    "Config file found using: 
                            DEFAULT Config !!!!!\n",
                    1,
                );
            }
        }
    }
    let statics_moved = statics.clone();
    HttpServer::new(move|| {
        App::new()
            .app_data(
            MultipartFormConfig::default()
                .total_limit(statics_moved.web_fwupdate_filesize * 1024 * 1024) // 120 MB
                .memory_limit(30 * 1024 * 1024), // 30 MB
            )
            .app_data(web::Data::new(statics_moved.clone()))
            .wrap(Logger::default())
            .service(
                web::resource(["/fwupdate/doFirmwareUpdate"])
                    .route(web::post().to(fwupdate_upload)),
            )
            .service(web::resource(["/","/fwupdate"]).route(web::get().to(fwupdate_get)))
    })
    .workers(2)
    .bind((statics.clone().web_bind_address, statics.clone().web_bind_port))?
    .run()
    .await
    //.map_err(anyhow::Error::from)
}

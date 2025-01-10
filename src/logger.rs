use std::io::Write;

use env_logger::{fmt::Color, Builder, Env};
use log::{debug, error, info, warn};

#[allow(dead_code)]
pub fn init_logger_default() {
    let env = Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");

    Builder::from_env(env)
        .format_level(false)
        .format_timestamp_nanos()
        .init();
}

pub fn init_logger_custom_format() {
    let env = Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");

    Builder::from_env(env)
        .format(move |buf, record| {
            let mut style_level = buf.style();

            let timestamp = buf.timestamp_seconds();

            match record.level() {
                log::Level::Info => style_level.set_color(Color::Yellow),
                log::Level::Debug => style_level.set_color(Color::Cyan),
                log::Level::Error => style_level.set_color(Color::Red),
                _ => style_level.set_color(Color::White),
            };

            let mut my_record = record.clone().args().to_string();
            let mut from = "main".to_string();
            let mut style_prefix = buf.style();
            if my_record.contains("%controller%") {
                style_prefix.set_color(Color::Green);
                my_record = my_record.replace("%controller%", "");
            }

            if my_record.contains("%web%") {
                style_prefix.set_color(Color::Cyan);
                my_record = my_record.replace("%web%", "");
                from = "web".to_string();
            }

            if my_record.contains("%wifi%") {
                style_prefix.set_color(Color::Cyan);
                my_record = my_record.replace("%wifi%", "");
                from = "wifi".to_string();
            }

            writeln!(
                buf,
                "{} {}: {}: {}",
                timestamp,
                style_prefix.value(from),
                style_level.value(record.level()),
                my_record.to_string(),
            )
        })
        .init();
}

#[allow(dead_code)]
fn find_string(from: u16) -> String {
    match from {
        0 => "main".to_string(),
        1 => "unknown".to_string(),
        2 => "unknown".to_string(),
        3 => "web".to_string(),
        4 => "unknown".to_string(),
        5 => "unknown".to_string(),
        6 => "wifi".to_string(),
        7 => "unknown".to_string(),
        8 => "sysinfo".to_string(),
        _ => "unknown".to_string(),
    }
}

#[allow(dead_code)]
pub fn ilog(msg: &str, from_id: u16) {
    let from = find_string(from_id);
    info!("%{}%{}", from, msg);
}

#[allow(dead_code)]
pub fn dlog(msg: &str, from_id: u16) {
    let from = find_string(from_id);
    debug!("%{}%{}", from, msg);
}

#[allow(dead_code)]
pub fn wlog(msg: &str, from_id: u16) {
    let from = find_string(from_id);
    warn!("%{}%{}", from, msg);
}

#[allow(dead_code)]
pub fn elog(msg: &str, from_id: u16) {
    let from = find_string(from_id);
    error!("%{}%{}", from, msg);
}
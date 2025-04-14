#[path = "../sysinfo.rs"]
mod sysinfo;

use std::time::Duration;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{Error, HttpResponse, Responder, web};

use tera::Tera;
use reqwest::Client;
use regex::Regex;

use crate::statics::Statics;


#[derive(Debug, MultipartForm)]
pub struct FirmwareUploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
    fw_filename: Text<String>,
}


async fn fetch_web_content(web_project_name: String, http_remote_addr: String, http_remote_fw_type: String ) -> std::result::Result<Vec<String>, reqwest::Error> {

    //let res = reqwest::get(format!("{}/fw/dev/{}/", http_remote_addr.clone(), web_project_name.clone())).await?;
    let url = format!("{}/fw/{}/{}/", http_remote_addr.clone(), http_remote_fw_type.clone(), web_project_name.clone());
    let res = Client::builder().timeout(Duration::new(2, 0)).build()?.get(url).send().await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;

    let href_regex = Regex::new(r#"href=\"(.+)\""#).unwrap();
    let mut links = Vec::new();

    for line in body.lines() {
        //println!("Line: {:?}", line);
        if let Some(captures) = href_regex.captures(line) {
            if let Some(link) = captures.get(1) {
                if link.as_str() != "../" {
                    links.push(link.as_str().to_string());
                }
            }
        }
    }

    println!("Links: {:?}", links);
    Ok(links)
}



pub async fn fwupdate(
    rauc_result: String,
    rauc_stdout: String,
    fw_list: Vec<String>,
) -> HttpResponse {
    let mut tera = Tera::new("src/webif/templates/*.html").unwrap();
    tera.add_raw_template("fwupdate", include_str!("templates/fwupdate.html"))
        .unwrap();

    let booted_slot = sysinfo::get_rauc_booted_slot()
                            .unwrap_or(
                                serde_json::from_str(r#"{"booted": "unknown"}"#).unwrap());
    let version = sysinfo::get_version().unwrap_or("unknown".to_string());
    
    // Prepare the context with some data
    let mut context = tera::Context::new();
    context.insert("hostname", &sysinfo::get_hostname());
    context.insert("bootedSlot", &booted_slot);
    context.insert("updateStatus", &rauc_result);
    context.insert("version", &version);
    context.insert("fwList", &fw_list);

    if rauc_result != "" {
        context.insert("getInfo", &rauc_stdout.lines().collect::<Vec<&str>>());
    }
    if rauc_result == "rebooting" {
        let _ = sysinfo::do_reboot();
    }

    HttpResponse::Ok().body(tera.render("fwupdate", &context).unwrap())
}

pub async fn fwupdate_get(statics: web::Data<Statics>) -> actix_web::Result<HttpResponse> {
    let rauc_result = String::from("");
    let rauc_stdout = String::from("");

    println!("fwupdate_get");
    println!("fwupdate_get - after fetch_web_content");
    
    let fw_list = match fetch_web_content(statics.web_project_name.clone(), statics.http_remote_addr.clone(), statics.http_remote_fw_type.clone() ).await {
        Ok(fw_list) => fw_list,
        Err(e) => {
            println!("Error fetching web content: {:?}", e);
            Vec::new()},
    };

    println!("{:?}", fw_list);

    Ok(fwupdate(rauc_result, rauc_stdout, fw_list).await)
}

pub async fn fwupdate_upload(
    MultipartForm(form): MultipartForm<FirmwareUploadForm>,
    statics: web::Data<Statics>
) -> Result<impl Responder, Error> {
    let mut rauc_result = String::from("");
    let mut rauc_stdout = String::from("");
    let fw_filename = form.fw_filename.clone();
    let mut files_list = form.files;
    let path: String;

    print!("Files: {:?}", files_list);
    println!("Filename: {:?}", fw_filename);

    if !fw_filename.is_empty() && fw_filename != "-" {
        path = format!(
            "{}/{}/{}/{}",
            statics.http_remote_addr.clone(),
            statics.http_remote_fw_type.clone(),
            statics.web_project_name.clone(),
            fw_filename.clone());
            println!("Remote path: {:?}", path);
    } else if !files_list.is_empty() {
        let file = files_list.pop().unwrap();
        let file_name = file.file_name.unwrap();
        path = format!("{}/{}",
            statics.web_fwupdate_path.clone(),
            file_name);
        println!("Local path: {:?}", path);
        if file.size > 0 {
            file.file.persist(path.clone()).unwrap();
        }
    } else {
        return Ok(HttpResponse::BadRequest().body("No file provided"));
    }
    
    if let Ok(res_string) = sysinfo::do_rauc_update(path) {
        if res_string.contains(" succeeded") {
            //println!("Update OK: {:?}", res_string.clone());
            rauc_result = String::from("rebooting");
        } else {
            //println!("Update Error: {:?}", res_string.clone());
            rauc_result = String::from("failed");
        }
        rauc_stdout = res_string.clone();
    }
    
    let fw_list = Vec::new();
    Ok(fwupdate(rauc_result, rauc_stdout, fw_list).await)
}

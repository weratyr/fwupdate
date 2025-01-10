#[path = "../sysinfo.rs"]
mod sysinfo;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};

use actix_web::{Error, HttpResponse, Responder};

use tera::Tera;

#[derive(Debug, MultipartForm)]
pub struct FirmwareUploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

pub async fn fwupdate(
    rauc_result: String,
    rauc_stdout: String,
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

    if rauc_result != "" {
        context.insert("getInfo", &rauc_stdout.lines().collect::<Vec<&str>>());
    }
    if rauc_result == "rebooting" {
        let _ = sysinfo::do_reboot();
    }

    HttpResponse::Ok().body(tera.render("fwupdate", &context).unwrap())
}

pub async fn fwupdate_get() -> actix_web::Result<HttpResponse> {
    let rauc_result = String::from("");
    let rauc_stdout = String::from("");
    Ok(fwupdate(rauc_result, rauc_stdout).await)
}

pub async fn fwupdate_upload(
    MultipartForm(form): MultipartForm<FirmwareUploadForm>,
) -> Result<impl Responder, Error> {
    let mut rauc_result = String::from("");
    let mut rauc_stdout = String::from("");

    //println!("fwupdate_upload: {:?}", form.files);

    for f in form.files {
        let file_name = f.file_name.unwrap();
        let path: String;

        if file_name == "" {
            break;
        }

        path = format!(
            "{}/{}",
            "/tmp",
            file_name
        );
        f.file.persist(path.clone()).unwrap();

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
    }

    Ok(fwupdate(rauc_result, rauc_stdout).await)
}

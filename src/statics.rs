
#[derive(Clone)]
pub struct Statics {
    pub web_bind_address: String,
    pub web_bind_port: u16,
    pub web_fwupdate_filesize: usize,
    pub web_fwupdate_path: String,
    pub web_project_name: String,
    pub http_remote_addr: String,
    pub http_remote_fw_type: String,
}

impl Statics {
    pub fn default() -> Statics {
        Statics {
            web_bind_address: "0.0.0.0".to_string(),
            web_bind_port: 80,
            web_fwupdate_filesize: 120,
            web_fwupdate_path: "/tmp".to_string(),
            web_project_name: "rollo".to_string(),
            http_remote_addr: "http://10.10.2.58:8181/".to_string(),
            http_remote_fw_type: "release".to_string(),
        }
    }

    pub fn new_from_file(config: config::Config) -> Statics {
            Statics {
            web_bind_address: config
                .get::<String>("web_bind_address")
                .unwrap_or("0.0.0.0".to_string()),
            web_bind_port: config.get("web_bind_port").unwrap(),
            web_fwupdate_filesize: config.get("web_fwupdate_filesize").unwrap(),
            web_fwupdate_path: config.get("web_fwupdate_path").unwrap(),
            web_project_name: config.get("web_project_name").unwrap(),
            http_remote_addr: config.get("http_remote_addr").unwrap(),
            http_remote_fw_type: config.get("http_remote_fw_type").unwrap(),
        }
    }
}

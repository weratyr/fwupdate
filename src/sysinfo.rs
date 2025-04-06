use serde_json::Value;


use std::io::Write;
use std::os::unix::fs::FileTypeExt;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, thread};
use std::process::{Command, Stdio};

use crate::logger::dlog;

#[allow(dead_code)]
pub fn get_hostname() -> String {
    let hostname = hostname::get();
    hostname.expect("unknown").to_string_lossy().to_string()
}

#[allow(dead_code)]
pub fn set_hostname(new_hostname: String) -> std::io::Result<()> {
    dlog(format!("set_hostname: {:?}", new_hostname).as_str(), 8);
    let hostname = hostname::set(new_hostname)?;
    dlog(format!("set_hostname: {:?}", hostname).as_str(), 8);

    Ok(())
}

#[allow(dead_code)]
pub fn persist_hostname(new_hostname: String) -> std::io::Result<()> {
    let _abc = write_to_file("/etc/hostname".to_string(), new_hostname);
    Ok(())
}

#[allow(dead_code)]
pub fn exec_cmd(cmd: &Vec<&str>) -> std::io::Result<String> {
    dlog(format!("sysinfo: exec_cmd: {:?}", cmd).as_str(), 0);
    let mut status = String::new();
    let cmd_out_status = Command::new(&cmd[0])
        .args(&cmd[1..])
        .stdout(Stdio::piped())
        .output()?;
    let cmd_out_str = String::from_utf8_lossy(&cmd_out_status.stdout);
    status.push_str(&cmd_out_str);

    Ok(status)
}

#[allow(dead_code)]
pub fn exec_cmd_exitcode(cmd: &Vec<&str>) -> Result<bool, String> {
    dlog(format!("sysinfo: exec_cmd_exitcode: {:?}", cmd).as_str(), 0);
    let exit_code = Command::new(&cmd[0])
        .args(&cmd[1..])
        .status()
        .map_err(|e| e.to_string())?
        .success();
    dlog(format!("sysinfo: exec_cmd_exitcode: {:?}", exit_code).as_str(), 0);
    Ok(exit_code)
}

#[allow(dead_code)]
pub fn get_rauc_booted_slot() -> Result<Value, String> {
    let args = &["status", "--output-format=json"];
    let prog = "/usr/bin/rauc";
    let v: Value;

    let rauc_get_status = Command::new(&prog)
            .arg(&args[0])
            .arg(&args[1])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()
            .map_err(|e| e.to_string())?;
        let rauc_status_str = String::from_utf8_lossy(&rauc_get_status.stdout);
        v = serde_json::from_str(&rauc_status_str).map_err(|e| e.to_string())?;

    Ok(v)
}

#[allow(dead_code)]
pub fn get_version() -> core::result::Result<String, String> {
    let version_file = "version";
    let version = std::fs::read_to_string(version_file)
        .map_err(|e| format!("Should have been able to read the file {}", e.to_string()))?;
    Ok(version)
}

#[allow(dead_code)]
pub fn write_to_file(file_path: String, content: String) -> core::result::Result<(), String> {
    let _ = std::fs::write(file_path, content)
        .map_err(|e| format!("Should have been able to write the file {}", e.to_string()))?;
    Ok(())
}

#[allow(dead_code)]
pub fn read_from_file(file_path: String) -> core::result::Result<String, String> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Should have been able to write the file {}", e.to_string()))?;
    Ok(content)
}

#[allow(dead_code)]
pub fn save_file(payload: &Vec<u8>, file_path: String) -> Option<bool> {
    let mut f: std::fs::File = std::fs::File::create(file_path).ok()?;
    // use chunks if the upload takes to long/ or use a thread
    //while let Some(chunk) = payload.next() {
    //    let data = chunk?;
    // Writing a file is also a blocking operation, so use a thread pool
    f = f.write_all(&payload).map(|_| f).ok()?;
    //}
    f.flush().ok()?;

    Some(true)
}

#[allow(dead_code)]
pub fn do_rauc_update(rauc_bundle: String) -> std::io::Result<String> {
    println!("do_rauc_update: {:?}", rauc_bundle);
    let mut rauc_install_cmd = Vec::new();
    rauc_install_cmd.push("rauc");
    rauc_install_cmd.push("install");
    rauc_install_cmd.push(&rauc_bundle);

    let res = exec_cmd(&rauc_install_cmd);
    res
}

#[allow(dead_code)]
pub fn do_reboot() -> std::io::Result<String> {
    _ = thread::spawn(move || {
        dlog("Rebooting in 5 seconds",0);
        sleep(Duration::from_millis(5000));
        let mut reboot_cmd = Vec::new();
        let cmd = format!("reboot");
        reboot_cmd.push(cmd.as_str());
        dlog("Rebooting",0);
        let res = exec_cmd(&reboot_cmd);
        res
    });
    Ok("Rebooting".to_string())
}

#[allow(dead_code)]
pub fn replace_string_unterscore(string_to_clean: String) -> String {
    string_to_clean
        .replace(|c| !char::is_alphanumeric(c), "_")
        .replace("_mp3", ".mp3")
}

#[allow(dead_code)]
pub fn replace_none_alphanumeric(string_to_clean: String) -> String {
    string_to_clean
        .replace(|c| !char::is_alphanumeric(c), "-")
}


#[allow(dead_code)]
pub fn start_squeezelite(blue_output: bool, squeezelite_name: String) -> Result<(), String> {
    //println!("start_dhcp_client: {:?}", interface);
    let mut cmd = [
        "/usr/bin/squeezelite",
        "-n",
        &squeezelite_name,
        "-z",
        "",
        "",
    ];
    if blue_output {
        cmd[4] = "-o";
        cmd[5] = "bluealsa";
    }
    let start_squeezelite = exec_cmd(&cmd.to_vec()).map_err(|e| e.to_string())?;
    println!("start_squeezelite: {:?}", start_squeezelite);
    Err(start_squeezelite)
}

#[allow(dead_code)]
pub fn killall_wpa_supplicant() -> Result<(), String> {
    let cmd = ["killall", "wpa_supplicant"];
    let res = exec_cmd(&cmd.to_vec()).map_err(|e| e.to_string())?;
    Err(res)
}

#[allow(dead_code)]
pub fn killall_squeezelite() -> Result<(), String> {
    let cmd = ["killall", "/usr/bin/squeezelite"];
    let stop_squeezelite = exec_cmd(&cmd.to_vec()).map_err(|e| e.to_string())?;
    Err(stop_squeezelite)
}

#[allow(dead_code)]
pub fn killall_dhcp_client() -> Result<(), String> {
    let cmd = ["killall", "udhcpc"];
    let stop_dhcp_client = exec_cmd(&cmd.to_vec()).map_err(|e| e.to_string())?;
    Err(stop_dhcp_client)
}

#[allow(dead_code)]
fn ping_ip(ip: String) -> Result<bool, String> {
    let cmd = ["/usr/bin/ping", "-c", "1", "-W", "1", &ip];
    let status = exec_cmd(&cmd.to_vec()).map_err(|e| e.to_string())?;

    match status.find("1 received") {
        Some(_) => {
            println!("ping_ip: true");
            Ok(true)
        }
        None => {
            println!("ping_ip: false");
            Err("false".to_string())
        }
    }
}
#[allow(dead_code)] // untested
fn is_named_pipe(path: &Path) -> Result<bool, String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let file_type = metadata.file_type();
    Ok(file_type.is_fifo())
}

#[allow(dead_code)]
pub fn is_named_socket(path: &String) -> Result<bool, String> {
    let path = Path::new(path);
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let file_type = metadata.file_type();
    Ok(file_type.is_socket())
}

#[allow(dead_code)]
pub fn file_exists_res(file: &String) -> Result<bool, String> {
    match Path::new(file).is_file() {
        true => Ok(true),
        false => Err(format!("File {} does not exist!", file)),
    }
}

#[allow(dead_code)]
pub fn dir_exists(dir: &String) -> bool {
    Path::new(dir).is_dir()
}

#[allow(dead_code)]
pub fn file_exists(file: &String) -> bool {
    Path::new(file).is_file()
}
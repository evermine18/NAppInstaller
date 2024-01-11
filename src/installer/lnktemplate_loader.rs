use std::{fs, path::{Path, PathBuf}};

pub fn lnkloader(template_dir: &str) -> Result<String, std::io::Error> {
    let lnktemplate = fs::read_to_string(template_dir)?;
    Ok(lnktemplate)
}

pub fn remplace_with_data(original_data : String, app_path : &str) -> String{
    let mut remplaced_data = original_data;
    let exec = app_path.to_string()+"/app.appimage";
    let icon = app_path.to_string()+"/squashfs-root/chatgpt-client.png"; // TODO : Change this to a more generic way
    remplaced_data = remplaced_data.replace("${name}", "App Test");
    remplaced_data = remplaced_data.replace("${exec}", &exec);
    remplaced_data = remplaced_data.replace("${icon_path}", &icon);
    remplaced_data
}
use std::{fs, path::{Path, PathBuf}};

pub fn lnkloader(template_dir: &str) -> Result<String, std::io::Error> {
    let lnktemplate = fs::read_to_string(template_dir)?;
    Ok(lnktemplate)
}

pub fn remplace_with_data(original_data : String, app_path : &PathBuf) -> String{
    let mut remplaced_data = original_data;
    let binding = app_path.join("app.appimage");
    let exec = binding.to_str().unwrap();
    remplaced_data = remplaced_data.replace("${name}", "App Test");
    remplaced_data = remplaced_data.replace("${exec}", &exec);
    remplaced_data = remplaced_data.replace("${icon_path}", &icon);
    remplaced_data
}
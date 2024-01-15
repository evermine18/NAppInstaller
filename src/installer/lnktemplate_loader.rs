use std::{fs, path::{Path, PathBuf}};

pub fn lnkloader(template_dir: &str) -> Result<String, std::io::Error> {
    let lnktemplate = fs::read_to_string(template_dir)?;
    Ok(lnktemplate)
}

pub fn remplace_with_data(original_data : String, app_path : &PathBuf) -> String{
    let mut remplaced_data = original_data;
    let binding = app_path.join("app.appimage");
    let exec = binding.to_str().unwrap();
    let mut icon = "";
    let mut icon = String::new();
    if let Ok(icon_paths) = search_icon_path(&app_path.join("squashfs-root")) {
        if let Some(icon_path) = icon_paths.get(0) {
            icon = icon_path.to_str().unwrap().to_string();
        }
    }
    remplaced_data = remplaced_data.replace("${name}", "App Test");
    remplaced_data = remplaced_data.replace("${exec}", &exec);
    remplaced_data = remplaced_data.replace("${icon_path}", &icon);
    remplaced_data
}

fn search_icon_path(app_path : &Path) -> std::io::Result<Vec<PathBuf>>{
    let mut png_files = Vec::new();
    println!("Finding at: {}",app_path.display());
    if app_path.is_dir() {
        for entry in fs::read_dir(app_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("png") {
                png_files.push(path);
            }
        }
    }

    Ok(png_files)
}
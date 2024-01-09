mod lnktemplate_loader;
use std::{fs::File, io::Write, env};
use lnktemplate_loader::lnkloader;

pub fn install(image_dir: &String){
    create_shortcut(image_dir);
}

fn create_shortcut(image_dir: &String){
    let mut shortcut = File::create("shortcut.lnk")
    .expect("Unable to create shortcut");
    let current_dir = env::current_dir().expect("Unable to get current directory");
    let template_path = current_dir.join("src/static/template.lnk");
    let mut data = lnkloader(&template_path.to_string_lossy().to_string()).unwrap_or("".to_string());
    shortcut.write_all(data.as_bytes()).expect("Unable to write to shortcut");
}
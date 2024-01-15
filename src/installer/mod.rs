mod lnktemplate_loader;
mod directory_tools;
use std::{fs::File, fs::create_dir_all, fs::copy, io::Write, env, process::Command, path::{Path, PathBuf}};
use lnktemplate_loader::{lnkloader, remplace_with_data};
use directory_tools as dt;

pub fn install(target: &String){
    let home_dir = dt::get_home_dir();
    let app_dir = create_app_directory(target,&home_dir);
    move_appimage(target, &app_dir);
    mount_appimage(&app_dir);
    create_shortcut(target, &app_dir);
    println!("{}",dt::get_home_dir().display());
}

fn create_shortcut(target: &String, app_path : &Path){
    let mut shortcut = File::create("shortcut.Desktop")
    .expect("Unable to create shortcut");
    let current_dir = env::current_dir().expect("Unable to get current directory");
    let template_path = current_dir.join("src/static/template.Desktop");
    let mut data = lnkloader(&template_path.to_string_lossy().to_string()).unwrap_or("".to_string());
    data = remplace_with_data(data, &app_path.to_path_buf());
    shortcut.write_all(data.as_bytes()).expect("Unable to write to shortcut");
}

fn create_app_directory(target: &String, home_path : &Path) -> PathBuf{
    let appdir_path = home_path.join(Path::new(".apps/app_name"));
    match create_dir_all(&appdir_path) {
        Err(why) => {
            println!("! {:?}", why.kind());
            Path::new("").into()
        },
        Ok(_) => {
            Path::new(&appdir_path).into()
        },
    }
}

fn move_appimage(tarjet: &String, app_path : &Path){
    let current_dir = env::current_dir().expect("Unable to get current directory"); // Temporary
    let appimage_path = app_path.join("app.appimage");
    match copy(tarjet, appimage_path) {
        Ok(_) => println!("AppImage copied!"),
        Err(e) => println!("Error while copying: {}", e),
    }
}

fn mount_appimage(app_path : &Path){
    let appimage_path = app_path.join("app.appimage");
    println!("{}", appimage_path.to_string_lossy());
    // mount -o loop appimage appdir
    Command::new(appimage_path)
        .arg("--appimage-extract")
        .current_dir(app_path)
        .status();
}
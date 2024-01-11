mod lnktemplate_loader;
use std::{fs::File, fs::create_dir_all, fs::copy, io::Write, env, process::Command};
use lnktemplate_loader::{lnkloader, remplace_with_data};

pub fn install(target: &String){
    let appdir_path = create_app_directory(target);
    create_shortcut(target);
    move_appimage(target);
    mount_appimage(&appdir_path,target);
}

fn create_shortcut(target: &String){
    let mut shortcut = File::create("shortcut.lnk")
    .expect("Unable to create shortcut");
    let current_dir = env::current_dir().expect("Unable to get current directory");
    let template_path = current_dir.join("src/static/template.lnk");
    let mut data = lnkloader(&template_path.to_string_lossy().to_string()).unwrap_or("".to_string());
    data = remplace_with_data(data);
    shortcut.write_all(data.as_bytes()).expect("Unable to write to shortcut");
}

fn create_app_directory(target: &String) -> String{

    let current_dir = env::current_dir().expect("Unable to get current directory"); // Temporary
    let appdir_path = current_dir.join("app_name");
    match create_dir_all(&appdir_path) {
        Err(why) => {
            println!("! {:?}", why.kind());
            "".to_string()
        },
        Ok(_) => {
            appdir_path.to_string_lossy().to_string()
        },
    }
}

fn move_appimage(tarjet: &String){
    let current_dir = env::current_dir().expect("Unable to get current directory"); // Temporary
    let appdir_path = current_dir.join("app_name/app.AppImage");

    match copy(tarjet, appdir_path) {
        Ok(_) => println!("AppImage copied!"),
        Err(e) => println!("Error while copying: {}", e),
    }
}

fn mount_appimage(appdir_path : &String, target : &String){
    // TODO : Change the way to obtain the AppImage path, without skipping the first character.
    let appimage_path = format!("{}{}", appdir_path, target.chars().skip(1).collect::<String>()); 
    println!("{}", appimage_path);
    // mount -o loop appimage appdir
    Command::new(appimage_path)
        .arg("--appimage-extract")
        .current_dir(appdir_path)
        .status();
}
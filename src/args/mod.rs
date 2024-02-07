mod core;

pub fn is_install() -> bool{
    // Checks if the --install flag is present
    let installed_arg = String::from("--install");
    core::argument_exists(installed_arg)
}

pub fn get_appimage_path() -> String{
    // Gets the appimage path
    let app_path = core::get_argument(1);
    app_path
}
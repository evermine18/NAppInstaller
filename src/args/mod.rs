mod core;

pub fn is_install(){
    // Checks if the --install flag is present
    let installed_arg = String::from("--install");
    println!("{}",core::argument_exists(installed_arg));
}

fn install(&image_dir: String){

}

fn create_shortcut(&image_dir: String){
    let mut shortcut = File::create("shortcut.lnk")
    .expect("Unable to create shortcut");
    file.write_all(b"[Desktop Entry]");
}
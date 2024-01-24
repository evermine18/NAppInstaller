use std::env;
use std::process::Command;

fn run_app() {
    let args: Vec<String> = env::args().collect();

    // check if the --install flag is present
    if args.contains(&"--install".to_string()) {
        println!("Installing...");
    } else {
        // Runnning the AppImage
        let output = Command::new("path_to_appimage")
            .output()
            .expect("failed to execute process");

        println!("Standard output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Error output: {}", String::from_utf8_lossy(&output.stderr));
    }
}
mod core;
use core::get_argument;

pub fn is_install() -> bool{
    // Checks if the --install flag is present
    args.contains(&"--install".to_string())
}
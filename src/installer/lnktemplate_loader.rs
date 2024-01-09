use std::fs;

pub fn lnkloader(template_dir: &str) -> Result<String, std::io::Error> {
    let lnktemplate = fs::read_to_string(template_dir)?;
    Ok(lnktemplate)
}
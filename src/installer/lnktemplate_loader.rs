use std::fs;

pub fn lnkloader(template_dir: &str) -> Result<String, std::io::Error> {
    let lnktemplate = fs::read_to_string(template_dir)?;
    Ok(lnktemplate)
}

pub fn remplace_with_data(original_data : String) -> String{
    let mut remplaced_data = original_data;
    remplaced_data = remplaced_data.replace("${name}", "App Test");
    remplaced_data = remplaced_data.replace("${target}", "C:\\Windows\\System32\\notepad.exe");
    remplaced_data
}
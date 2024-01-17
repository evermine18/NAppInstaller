use std::ffi::c_char;

#[link(name = "appimage")]
extern "C" {
    fn appimage_list_files(path: *const c_char) -> *mut *mut c_char;
}
pub fn test_lib(){
    let path = "app.AppImage";
    unsafe { 
        let files = appimage_list_files(path.as_ptr() as *const c_char);

        // Handle the result
        if files.is_null() {
            println!("Error: the function returned NULL.");
        } else{
            // TEST
            // let test = std::ffi::CStr::from_ptr(*files);
            // println!("LOG: {}", test.to_string_lossy().into_owned());
            let mut i = 0;
            while !(*files.offset(i)).is_null() {
                let file_cstr = std::ffi::CStr::from_ptr(*files.offset(i));
                println!("File: {}", file_cstr.to_string_lossy().into_owned());
                i += 1;
            }
        }
        
    }
}
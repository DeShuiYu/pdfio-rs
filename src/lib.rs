include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn file_open(filename:&str)  {
    unsafe {

        let filename_cstr = std::ffi::CString::new(filename).unwrap();
        let filename_ptr = filename_cstr.as_ptr();

        let password_cb = None;

        let password_data: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        let error_cb = None;
        let error_data: *mut ::std::os::raw::c_void = std::ptr::null_mut();

        let f =  pdfioFileOpen(filename_ptr, password_cb, password_data, error_cb, error_data);
    }
}


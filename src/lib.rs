#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use leptonica_sys::*;

include!(concat!(env!("OUT_DIR"), "/capi_bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/public_types_bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use leptonica_sys::{pixFreeData, pixRead};
    use std::ffi::CStr;
    use std::ptr;
    use std::str;

    #[test]
    fn ocr() {
        unsafe {
            let cube = TessBaseAPICreate();
            TessBaseAPIInit3(
                cube,
                ptr::null(),
                CStr::from_bytes_with_nul_unchecked(b"eng\0").as_ptr(),
            );
            let image = pixRead(CStr::from_bytes_with_nul_unchecked(b"img.png\0").as_ptr());
            TessBaseAPISetImage2(cube, image);
            TessBaseAPIRecognize(cube, ptr::null_mut());
            assert_eq!(
                str::from_utf8(CStr::from_ptr(TessBaseAPIGetUTF8Text(cube)).to_bytes()),
                Ok(include_str!("../img.txt"))
            );
            pixFreeData(image);
            TessBaseAPIDelete(cube);
        }
    }

    #[test]
    #[allow(path_statements)]
    fn defined_constants() {
        kMinCredibleResolution;
        kMaxCredibleResolution;
    }
}

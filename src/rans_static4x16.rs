extern "C" {
    #[must_use]
    pub fn rans_compress_bound_4x16(
        size: ::std::os::raw::c_uint,
        order: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[must_use]
    pub fn rans_compress_to_4x16(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out: *mut ::std::os::raw::c_uchar,
        out_size: *mut ::std::os::raw::c_uint,
        order: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn rans_compress_4x16(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out_size: *mut ::std::os::raw::c_uint,
        order: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn rans_uncompress_to_4x16(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out: *mut ::std::os::raw::c_uchar,
        out_size: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn rans_uncompress_4x16(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out_size: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar;
}

extern "C" {
    #[must_use]
    pub fn rans_compress(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out_size: *mut ::std::os::raw::c_uint,
        order: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn rans_uncompress(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out_size: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar;
}

extern "C" {
    #[must_use]
    pub fn arith_compress(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out_size: *mut ::std::os::raw::c_uint,
        order: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn arith_uncompress(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out_size: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn arith_compress_to(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out: *mut ::std::os::raw::c_uchar,
        out_size: *mut ::std::os::raw::c_uint,
        order: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn arith_uncompress_to(
        in_: *mut ::std::os::raw::c_uchar,
        in_size: ::std::os::raw::c_uint,
        out: *mut ::std::os::raw::c_uchar,
        out_sz: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    #[must_use]
    pub fn arith_compress_bound(
        size: ::std::os::raw::c_uint,
        order: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}

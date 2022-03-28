extern "C" {
    #[must_use]
    pub fn encode_names(
        blk: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        level: ::std::os::raw::c_int,
        use_arith: ::std::os::raw::c_int,
        out_len: *mut ::std::os::raw::c_int,
        last_start_p: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    #[must_use]
    pub fn decode_names(in_: *mut u8, sz: u32, out_len: *mut u32) -> *mut u8;
}

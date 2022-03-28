use libc::size_t;

pub const FQZ_FREVERSE: u32 = 16;
pub const FQZ_FREAD2: u32 = 128;
pub const FQZ_VERS: u32 = 5;
pub const FQZ_MAX_STRAT: u32 = 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fqz_slice {
    pub num_records: ::std::os::raw::c_int,
    pub len: *mut u32,
    pub flags: *mut u32,
}
#[test]
fn bindgen_test_layout_fqz_slice() {
    assert_eq!(
        ::std::mem::size_of::<fqz_slice>(),
        24usize,
        concat!("Size of: ", stringify!(fqz_slice))
    );
    assert_eq!(
        ::std::mem::align_of::<fqz_slice>(),
        8usize,
        concat!("Alignment of ", stringify!(fqz_slice))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_slice>())).num_records as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_slice),
            "::",
            stringify!(num_records)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_slice>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_slice),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_slice>())).flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_slice),
            "::",
            stringify!(flags)
        )
    );
}

pub const GFLAG_MULTI_PARAM: ::std::os::raw::c_int = 1;
pub const GFLAG_HAVE_STAB: ::std::os::raw::c_int = 2;
pub const GFLAG_DO_REV: ::std::os::raw::c_int = 4;

pub const PFLAG_DO_DEDUP: ::std::os::raw::c_int = 2;
pub const PFLAG_DO_LEN: ::std::os::raw::c_int = 4;
pub const PFLAG_DO_SEL: ::std::os::raw::c_int = 8;
pub const PFLAG_HAVE_QMAP: ::std::os::raw::c_int = 16;
pub const PFLAG_HAVE_PTAB: ::std::os::raw::c_int = 32;
pub const PFLAG_HAVE_DTAB: ::std::os::raw::c_int = 64;
pub const PFLAG_HAVE_QTAB: ::std::os::raw::c_int = 128;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fqz_param {
    pub context: u16,
    pub pflags: ::std::os::raw::c_uint,
    pub do_sel: ::std::os::raw::c_uint,
    pub do_dedup: ::std::os::raw::c_uint,
    pub store_qmap: ::std::os::raw::c_uint,
    pub fixed_len: ::std::os::raw::c_uint,
    pub use_qtab: ::std::os::raw::c_uchar,
    pub use_dtab: ::std::os::raw::c_uchar,
    pub use_ptab: ::std::os::raw::c_uchar,
    pub qbits: ::std::os::raw::c_uint,
    pub qloc: ::std::os::raw::c_uint,
    pub pbits: ::std::os::raw::c_uint,
    pub ploc: ::std::os::raw::c_uint,
    pub dbits: ::std::os::raw::c_uint,
    pub dloc: ::std::os::raw::c_uint,
    pub sbits: ::std::os::raw::c_uint,
    pub sloc: ::std::os::raw::c_uint,
    pub max_sym: ::std::os::raw::c_int,
    pub nsym: ::std::os::raw::c_int,
    pub max_sel: ::std::os::raw::c_int,
    pub qmap: [::std::os::raw::c_uint; 256usize],
    pub qtab: [::std::os::raw::c_uint; 256usize],
    pub ptab: [::std::os::raw::c_uint; 1024usize],
    pub dtab: [::std::os::raw::c_uint; 256usize],
    pub qshift: ::std::os::raw::c_int,
    pub pshift: ::std::os::raw::c_int,
    pub dshift: ::std::os::raw::c_int,
    pub sshift: ::std::os::raw::c_int,
    pub qmask: ::std::os::raw::c_uint,
    pub do_r2: ::std::os::raw::c_int,
    pub do_qa: ::std::os::raw::c_int,
}

#[test]
fn bindgen_test_layout_fqz_param() {
    assert_eq!(
        ::std::mem::size_of::<fqz_param>(),
        7268usize,
        concat!("Size of: ", stringify!(fqz_param))
    );
    assert_eq!(
        ::std::mem::align_of::<fqz_param>(),
        4usize,
        concat!("Alignment of ", stringify!(fqz_param))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).context as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).pflags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(pflags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).do_sel as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(do_sel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).do_dedup as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(do_dedup)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).store_qmap as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(store_qmap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).fixed_len as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(fixed_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).use_qtab as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(use_qtab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).use_dtab as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(use_dtab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).use_ptab as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(use_ptab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).qbits as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(qbits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).qloc as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(qloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).pbits as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(pbits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).ploc as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(ploc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).dbits as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(dbits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).dloc as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(dloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).sbits as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(sbits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).sloc as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(sloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).max_sym as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(max_sym)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).nsym as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(nsym)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).max_sel as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(max_sel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).qmap as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(qmap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).qtab as *const _ as usize },
        1096usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(qtab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).ptab as *const _ as usize },
        2120usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(ptab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).dtab as *const _ as usize },
        6216usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(dtab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).qshift as *const _ as usize },
        7240usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(qshift)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).pshift as *const _ as usize },
        7244usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(pshift)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).dshift as *const _ as usize },
        7248usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(dshift)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).sshift as *const _ as usize },
        7252usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(sshift)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).qmask as *const _ as usize },
        7256usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(qmask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).do_r2 as *const _ as usize },
        7260usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(do_r2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_param>())).do_qa as *const _ as usize },
        7264usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_param),
            "::",
            stringify!(do_qa)
        )
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fqz_gparams {
    pub vers: ::std::os::raw::c_int,
    pub gflags: ::std::os::raw::c_uint,
    pub nparam: ::std::os::raw::c_int,
    pub max_sel: ::std::os::raw::c_int,
    pub stab: [::std::os::raw::c_uint; 256usize],
    pub max_sym: ::std::os::raw::c_int,
    pub p: *mut fqz_param,
}

#[test]
fn bindgen_test_layout_fqz_gparams() {
    assert_eq!(
        ::std::mem::size_of::<fqz_gparams>(),
        1056usize,
        concat!("Size of: ", stringify!(fqz_gparams))
    );
    assert_eq!(
        ::std::mem::align_of::<fqz_gparams>(),
        8usize,
        concat!("Alignment of ", stringify!(fqz_gparams))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_gparams>())).vers as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_gparams),
            "::",
            stringify!(vers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_gparams>())).gflags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_gparams),
            "::",
            stringify!(gflags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_gparams>())).nparam as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_gparams),
            "::",
            stringify!(nparam)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_gparams>())).max_sel as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_gparams),
            "::",
            stringify!(max_sel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_gparams>())).stab as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_gparams),
            "::",
            stringify!(stab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_gparams>())).max_sym as *const _ as usize },
        1040usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_gparams),
            "::",
            stringify!(max_sym)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fqz_gparams>())).p as *const _ as usize },
        1048usize,
        concat!(
            "Offset of field: ",
            stringify!(fqz_gparams),
            "::",
            stringify!(p)
        )
    );
}

extern "C" {
    #[doc = " Compress a block of quality values."]
    #[doc = ""]
    #[doc = " @param vers          The CRAM version number (<<8) plus fqz strategy (0-3)"]
    #[doc = " @param s             Length and flag data CRAM per-record"]
    #[doc = " @param in            Buffer of concatenated quality values (no separator)"]
    #[doc = " @param in_size       Size of in buffer"]
    #[doc = " @param out_size      Size of returned output"]
    #[doc = " @param strat         FQZ compression strategy (0 to FQZ_MAX_STRAT)"]
    #[doc = " @param gp            Optional fqzcomp paramters (may be NULL)."]
    #[doc = ""]
    #[doc = " @return              The compressed quality buffer on success,"]
    #[doc = "                      NULL on failure."]
    #[must_use]
    pub fn fqz_compress(
        vers: ::std::os::raw::c_int,
        s: *mut fqz_slice,
        in_: *mut ::std::os::raw::c_char,
        in_size: size_t,
        out_size: *mut size_t,
        strat: ::std::os::raw::c_int,
        gp: *mut fqz_gparams,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Decompress a block of quality values."]
    #[doc = ""]
    #[doc = " @param in            Buffer of compressed quality values"]
    #[doc = " @param in_size       Size of in buffer"]
    #[doc = " @param out_size      Size of returned output"]
    #[doc = " @param lengths       Optional array filled out with record lengths."]
    #[doc = "                      May be NULL.  If not, preallocate it to correct size."]
    #[doc = ""]
    #[doc = " @return              The uncompressed concatenated qualities on success,"]
    #[doc = "                      NULL on failure."]
    #[must_use]
    pub fn fqz_decompress(
        in_: *mut ::std::os::raw::c_char,
        in_size: size_t,
        out_size: *mut size_t,
        lengths: *mut ::std::os::raw::c_int,
        nlengths: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " A utlity function to analyse a quality buffer to gather statistical"]
    #[doc = "  information.  This is written into qhist and pm.  This function is only"]
    #[doc = "  useful if you intend on passing your own fqz_gparams block to"]
    #[doc = "  fqz_compress."]
    #[must_use]
    pub fn fqz_qual_stats(
        s: *mut fqz_slice,
        in_: *mut ::std::os::raw::c_uchar,
        in_size: size_t,
        pm: *mut fqz_param,
        qhist: *mut u32,
        one_param: ::std::os::raw::c_int,
    );
}

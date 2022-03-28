use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut c = cc::Build::new();
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let config_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("include");
    let vendor = dunce::canonicalize(root.join("vendor")).unwrap();

    fs::create_dir_all(&config_dir).unwrap();

    println!(
        "cargo:include={}",
        env::join_paths(&[&config_dir, &vendor])
            .unwrap()
            .to_str()
            .unwrap()
    );
    c.include(&config_dir);
    c.include(&vendor);
    c.pic(true);

    if let Ok(target_cpu) = env::var("TARGET_CPU") {
        c.flag_if_supported(&format!("-march={}", target_cpu));
    }

    c.warnings(false);

    let files = &[
        "vendor/htscodecs/rANS_static4x16pr.c",
        "vendor/htscodecs/rANS_static.c",
        "vendor/htscodecs/arith_dynamic.c",
        "vendor/htscodecs/fqzcomp_qual.c",
        "vendor/htscodecs/pack.c",
        "vendor/htscodecs/htscodecs.c",
        "vendor/htscodecs/rle.c",
        "vendor/htscodecs/tokenise_name3.c",
    ];

    for file in files.iter() {
        c.file(file);
    }

    let mut config_h = fs::File::create(config_dir.join("config.h")).unwrap();
    write!(
        config_h,
        r#"
            #define HAVE_DLFCN_H 1
            #define HAVE_FCNTL_H 1
            #define HAVE_INTTYPES_H 1
            #define HAVE_LIBBZ2 1
            /* #undef HAVE_LIBDEFLATE */
            #define HAVE_LIMITS_H 1
            #define HAVE_MALLOC_H 1
            #define HAVE_STDINT_H 1
            #define HAVE_STDIO_H 1
            #define HAVE_STDLIB_H 1
            #define HAVE_STRINGS_H 1
            #define HAVE_STRING_H 1
            #define HAVE_SYS_STAT_H 1
            #define HAVE_SYS_TYPES_H 1
            #define HAVE_SYS_WAIT_H 1
            #define HAVE_UNISTD_H 1
            #define HAVE_ZLIB 1
            
            #define LT_OBJDIR ".libs/"
            #define PACKAGE "{PACKAGE_NAME}"
            #define PACKAGE_BUGREPORT ""
            #define PACKAGE_NAME "{PACKAGE_NAME}"
            #define PACKAGE_STRING "{PACKAGE_NAME} {PACKAGE_VERSION}"
            #define PACKAGE_TARNAME "{PACKAGE_NAME}"
            #define PACKAGE_URL ""
            #define PACKAGE_VERSION "{PACKAGE_VERSION}"
            #define STDC_HEADERS 1
            #define VERSION "{PACKAGE_VERSION}"
        "#,
        PACKAGE_NAME = env::var("CARGO_PKG_NAME").unwrap(),
        PACKAGE_VERSION = env::var("CARGO_PKG_VERSION").unwrap()
    )
    .unwrap();

    drop(config_h); // close the file

    let mut version_h = fs::File::create(config_dir.join("version.h")).unwrap();
    write!(
        version_h,
        r#"
            #define HTSCODECS_VERSION_TEXT "{PACKAGE_VERSION}"
        "#,
        PACKAGE_VERSION = env::var("CARGO_PKG_VERSION").unwrap()
    )
    .unwrap();

    drop(version_h); // close the file

    c.compile("htscodecs");
}

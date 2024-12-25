use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    cc::Build::new()
        .includes([
            "pdfio/pdfio-content.h",
            "pdfio/pdfio-private.h",
            "pdfio/pdfio.h",
            "pdfio/ttf.h",
        ])
        .files([
            "pdfio/pdfio-aes.c",
            "pdfio/pdfio-array.c",
            "pdfio/pdfio-common.c",
            "pdfio/pdfio-content.c",
            "pdfio/pdfio-crypto.c",
            "pdfio/pdfio-dict.c",
            "pdfio/pdfio-file.c",
            "pdfio/pdfio-md5.c",
            "pdfio/pdfio-object.c",
            "pdfio/pdfio-page.c",
            "pdfio/pdfio-rc4.c",
            "pdfio/pdfio-sha256.c",
            "pdfio/pdfio-stream.c",
            "pdfio/pdfio-string.c",
            "pdfio/pdfio-token.c",
            "pdfio/pdfio-value.c",
            "pdfio/pdfiototext.c",
            "pdfio/testpdfio.c",
            "pdfio/testttf.c",
            "pdfio/ttf.c",
        ])
        .flag("-Wno-unused-parameter")
        .flag("-L/opt/homebrew/opt/zlib/lib")
        .flag("-I/opt/homebrew/opt/zlib/include")
        .out_dir(&out_path)
        .compile("pdfio");
    println!("cargo:rustc-link-lib=static=pdfio"); // 链接静态库 libpdfio.a
    println!("cargo:rustc-link-search=native={}", out_path.display()); // 设置库路径
    let bindings = bindgen::Builder::default()
        .headers([
            "pdfio/pdfio-content.h",
            "pdfio/pdfio-private.h",
            "pdfio/pdfio.h",
            "pdfio/ttf.h",
        ])
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

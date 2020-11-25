extern crate cc;
extern crate cmake;

fn main() {
    // Compile and link SEAL
    let dst = cmake::Config::new("seal/native/src")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .build();

    cc::Build::new()
        .file("sealpir/pir.cpp")
        .file("sealpir/pir_server.cpp")
        .file("sealpir/pir_client.cpp")
        .file("sealpir-bindings/pir_rust.cpp")
        .include("sealpir-bindings/")
        .include("sealpir/")
        .include(&format!("{}/include/", dst.display()))
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .flag("-std=c++17")
        .flag("-fopenmp")
        .pic(true)
        .cpp(true)
        .compile("libsealpir.a");

    println!("cargo:rustc-link-search={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=seal");
}

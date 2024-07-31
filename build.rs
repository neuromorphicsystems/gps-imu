fn main() {
    let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
    let header_path = current_dir.join("libft4222.h");
    let vendor_path = current_dir.join("vendor");
    let search_path = match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" => vendor_path.join("windows").join("amd64"),
            "x86" => vendor_path.join("windows").join("i386"),
            target_arch => panic!("Target architecture not supported: {target_arch}"),
        },
        "linux" => match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" => vendor_path.join("linux").join("x86_64"),
            "arm" | "aarch64" => match std::env::var("TARGET").unwrap().as_str() {
                "arm-unknown-linux-musleabihf" | "arm-unknown-linux-gnueabihf" => {
                    vendor_path.join("linux").join("armv6-hf")
                }
                "armv7-unknown-linux-musleabihf" | "armv7-unknown-linux-gnueabihf" => {
                    vendor_path.join("linux").join("armv7-hf")
                }
                "aarch64-unknown-linux-musl" | "aarch64-unknown-linux-gnu" => {
                    vendor_path.join("linux").join("armv8")
                }
                target => panic!("Target not supported: {target}"),
            },
            target_arch => panic!("Target architecture not supported: {target_arch}"),
        },
        "macos" => match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" | "aarch64" => vendor_path.join("macos"),
            target_arch => panic!("Target architecture not supported: {target_arch}"),
        },
        target_os => panic!("Target OS not supported: {target_os}"),
    };
    println!(
        "cargo:rustc-link-search=native={}",
        search_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=ft4222");
    match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=c++.1");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=c++.1");
        }
        "windows" => {}
        target_os => panic!("Target OS not supported: {target_os}"),
    }
    println!("cargo:rerun-if-changed={}", header_path.to_str().unwrap());
    println!("cargo:rerun-if-env-changed=LIBMSVC_PATH");
    bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .allowlist_function("FT(4222)?_.*")
        .allowlist_type("FT(4222)?_.*")
        .allowlist_var("FT(4222)?_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("ft4222_bindings.rs"),
        )
        .expect("Unable to write bindings");
}

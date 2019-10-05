
use bindgen;
use cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let v8_dir = match env::var("CUSTOM_V8") {
        Ok(custom_v8_dir) => {
            let custom_v8_dir = PathBuf::from(custom_v8_dir);
            assert!(custom_v8_dir.exists());
            custom_v8_dir
        }
        Err(_) => panic!("Must use CUSTOM_V8 env_var"),
    };
    #[cfg(target_env = "msvc")] {
        compile_wrappers_windows(v8_dir.clone());
        generate_bindings_windows(v8_dir.clone());
    }
    #[cfg(not(target_env = "msvc"))] {
        compile_wrappers_not_windows(v8_dir.clone());
        generate_bindings_not_windows(v8_dir);
    }
}

fn compile_wrappers_not_windows(v8_dir: PathBuf) {
    let include_dir = v8_dir.join("include");

    println!("cargo:rerun-if-changed=src/wrapper.cpp");

    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("--std=c++14")
        .include(include_dir)
        .file("src/wrapper.cpp")
        .compile("libwrapper.a");
}

fn generate_bindings_not_windows(v8_dir: PathBuf) {
    println!("cargo:rustc-link-lib=v8_libbase");
    println!("cargo:rustc-link-lib=v8_libplatform");
    println!("cargo:rustc-link-lib=v8_monolith");
    println!("cargo:rustc-link-lib=c++");
    println!(
        "cargo:rustc-link-search={}/out.gn/x64.release/obj",
        v8_dir.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search={}/out.gn/x64.release/obj/third_party/icu",
        v8_dir.to_str().unwrap()
    );

    let bindings = bindgen::Builder::default()
        .generate_comments(true)
        .header("src/wrapper.cpp")
        .rust_target(bindgen::RustTarget::Nightly)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("--std=c++14")
        .clang_arg(format!("-I{}", v8_dir.join("include").to_str().unwrap()))
        // Because there are some layout problems with these
        .opaque_type("std::.*")
        .whitelist_type("std::unique_ptr\\<v8::Platform\\>")
        .whitelist_type("v8::.*")
        .blacklist_type("std::basic_string.*")
        .whitelist_function("v8::.*")
        .whitelist_function("osgood::.*")
        .whitelist_var("v8::.*")
        // Re-structure the modules a bit and hide the "root" module
        .raw_line("#[doc(hidden)]")
        // .generate_inline_functions(true)
        .enable_cxx_namespaces()
        .derive_debug(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .rustfmt_bindings(true) // comment this for a slightly faster build
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}


fn compile_wrappers_windows(v8_dir: PathBuf) {
    let include_dir = v8_dir.join("include");

    println!("cargo:rerun-if-changed=src/wrapper.cpp");

    cc::Build::new()
        .flag("/EHsc")
        .warnings_into_errors(false)
        .warnings(false)
        .flag("/std:c++14")
        .cpp(true)
        .include(include_dir)
        .file("src/wrapper.cpp")
        .compile("wrapper");
}

fn generate_bindings_windows(v8_dir: PathBuf) {

    println!("cargo:rustc-link-lib=DbgHelp");
    println!("cargo:rustc-link-lib=Winmm");
    println!("cargo:rustc-link-lib=Shlwapi");


    println!("cargo:rustc-link-lib=v8_libbase");
    println!("cargo:rustc-link-lib=v8_libplatform");
    println!("cargo:rustc-link-lib=v8_monolith");
    println!(
        "cargo:rustc-link-search={}/lib",
        v8_dir.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search={}/lib/third_party/icu",
        v8_dir.to_str().unwrap()
    );

    println!(
        "cargo:rustc-link-search={}/out.gn/x64.release/obj",
        v8_dir.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search={}/out.gn/x64.release/obj/third_party/icu",
        v8_dir.to_str().unwrap()
    );


    let bindings = bindgen::Builder::default()
        .generate_comments(true)
        .header("src/wrapper.cpp")
        .rust_target(bindgen::RustTarget::Nightly)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("--std=c++14")
        .clang_arg(format!("-I{}", v8_dir.join("include").to_str().unwrap()))
        // Because there are some layout problems with these
        .opaque_type("std::.*")
        .whitelist_type("std::unique_ptr\\<v8::Platform\\>")
        .whitelist_type("v8::.*")
        .blacklist_type("std::basic_string.*")
        .whitelist_function("v8::.*")
        .whitelist_function("osgood::.*")
        .whitelist_var("v8::.*")
        // Re-structure the modules a bit and hide the "root" module
        .raw_line("#[doc(hidden)]")
        // .generate_inline_functions(true)
        .enable_cxx_namespaces()
        .derive_debug(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .rustfmt_bindings(true) // comment this for a slightly faster build
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

use build_script::cargo_rerun_if_changed;
use std::path::PathBuf;
use std::{env, path::Path};

fn main() -> miette::Result<()> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir = Path::new(&cargo_manifest_dir);
    let duckdb_root = cargo_manifest_dir
        .join("duckdb")
        .canonicalize()
        .expect("duckdb source root");

    let header = "src/wrapper.h";

    #[cfg(feature = "statically_linked")]
    {
        use build_script::{cargo_rustc_link_lib, cargo_rustc_link_search};
        cargo_rustc_link_lib("duckdb");
        cargo_rustc_link_search(duckdb_root.join("build/debug/src"));
        cargo_rustc_link_search(duckdb_root.join("build/release/src"));
    }

    let main_file = "src/defs.rs";
    let duckdb = duckdb_root.join("src/include");
    let src = cargo_manifest_dir.join("src");

    let mut b = autocxx_build::Builder::new(main_file, [&duckdb, &src])
        .build()
        .expect("autocxx");
    let wrapper = "src/wrapper.cpp";
    b.include(&duckdb)
        .include(&src)
        .files(vec![wrapper])
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-redundant-move")
        .flag_if_supported("-std=c++14")
        .compile("autocxx-demo"); // arbitrary library name, pick anything
    cargo_rerun_if_changed(main_file);
    cargo_rerun_if_changed(wrapper);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    cargo_rerun_if_changed(header);
    cargo_rerun_if_changed(&(header.to_owned() + "pp"));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header)
        // .enable_cxx_namespaces()
        // .generate_comments(true)
        // .derive_default(true)
        // Tell bindgen we are processing c++
        // .clang_arg("-xc++")
        // .clang_arg("-std=c++11")
        .clang_arg("-I")
        .clang_arg(duckdb.to_string_lossy())
        // .allowlist_type("duckdb::DuckDB")
        // .opaque_type("std::.*")
        .derive_debug(true)
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

use bindgen;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/density_api.h");
    println!("cargo:rerun-if-changed=src/c_bindings.rs");

    // create rust bindings for the C portion of Density using bindgen
    let bindings = bindgen::Builder::default()
        .header("src/density_api.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //Use core instead of libstd in the generated bindings.
        .use_core()
        // use cty as the prefix for the raw types instead of ::std::os::raw
        .ctypes_prefix("cty")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("src/c_bindings.rs").expect("Couldn't write bindings!");
    
    // compile the C portion of Density using the cc crate
    cc::Build::new()
        .file("src/globals.c")
        .file("src/structure/header.c")
        .file("src/buffers/buffer.c")
        .file("src/algorithms/algorithms.c")
        .file("src/algorithms/dictionaries.c")
        .file("src/algorithms/chameleon/core/chameleon_decode.c")
        .file("src/algorithms/chameleon/core/chameleon_encode.c")
        .file("src/algorithms/cheetah/core/cheetah_decode.c")
        .file("src/algorithms/cheetah/core/cheetah_encode.c")
        .file("src/algorithms/lion/core/lion_decode.c")
        .file("src/algorithms/lion/core/lion_encode.c")
        .file("src/algorithms/lion/forms/lion_form_model.c")
        .static_flag(true)
        .debug(true)
        .compile("density_c");

    // link the compiled static library
    println!("cargo:rustc-link-lib=static=density_c");
}
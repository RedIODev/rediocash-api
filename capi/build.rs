use std::{path::PathBuf, str::FromStr};

fn main() {
    let bindings = bindgen::Builder::default()
            .header("./header/rediocash_capi.h")
        //     .ignore_functions()
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings!");
    let out_path = PathBuf::from_str("./src/capi.rs")
            .expect("Project structure incorrect");
    bindings.write_to_file(out_path)
            .expect("Couldn't write bindings!");
}
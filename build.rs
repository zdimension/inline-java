use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:warning={}", env::var_os("OUT_DIR").unwrap().into_string().unwrap());
    jerk::metabuild();
    std::fs::copy(
        Path::new(&env::var("OUT_DIR").unwrap()).join("java/jars/inline-java.jar"),
        get_output_path().join("inline-java.jar"))
        .expect("couldn't copy JAR");
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}
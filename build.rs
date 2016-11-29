
// https://serde.rs/codegen-hybrid.html

#[cfg(feature = "serde_codegen")]
fn main() {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");
    serde_codegen::expand(&src, &dst).unwrap();

    let src = Path::new("src/merge_requests/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("merge_requests");
    std::fs::create_dir_all(&dst).expect(&format!("Cannot create directory {:?}!", dst));
    let dst = dst.join("serde_types.rs");
    serde_codegen::expand(&src, &dst).unwrap();
}

#[cfg(not(feature = "serde_codegen"))]
fn main() {
    // do nothing
}

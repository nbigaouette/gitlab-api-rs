
// https://serde.rs/codegen-hybrid.html

fn module_codegen(out_dir: &std::ffi::OsString, module_name: &str) {
    extern crate serde_codegen;

    use std::path::Path;

    let src = Path::new("src").join(module_name).join("serde_types.in.rs");
    let dst = Path::new(&out_dir).join(module_name);
    std::fs::create_dir_all(&dst).expect(&format!("Cannot create directory {:?}!", dst));
    let dst = dst.join("serde_types.rs");
    serde_codegen::expand(&src, &dst).unwrap();
}


#[cfg(feature = "serde_codegen")]
fn main() {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");
    serde_codegen::expand(&src, &dst).unwrap();

    module_codegen(&out_dir, "groups");
    module_codegen(&out_dir, "issues");
    module_codegen(&out_dir, "merge_requests");
    module_codegen(&out_dir, "projects");
}

#[cfg(not(feature = "serde_codegen"))]
fn main() {
    // do nothing
}

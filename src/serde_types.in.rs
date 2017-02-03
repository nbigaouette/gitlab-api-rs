
// https://users.rust-lang.org/t/what-am-i-doing-wrong-go-program-is-12x-faster-than-rust/5692/13
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
// https://serde.rs/codegen-hybrid.html


// FIXME: Use https://github.com/nox/serde_urlencoded
// FIXME: Use a type for sha1
// FIXME: Use chrono crate for dates
// FIXME: Use unsigned integers where it makes sense (id, iid, etc.)
// FIXME: Verify all `match` in push_str() in build_query(): They should contain all members.
// FIXME: Get rid of build_query(), use serde's Serialize instead.

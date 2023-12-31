use std::ffi::c_long;

#[link(name = "foo")]
extern "C" {
    // This declaration would have the type id "_ZTSFvlE", but at the time types
    // are encoded, all type aliases are already resolved to their respective
    // Rust aliased types, so this is encoded either as "_ZTSFvu3i32E" or
    // "_ZTSFvu3i64E" depending to what type c_long type alias is resolved to,
    // which currently uses the u<length><type-name> vendor extended type
    // encoding for the Rust integer types--this is the problem demonstrated in
    // this example.
    fn hello_from_c(_: c_long);

    // This declaration would have the type id "_ZTSFvPFvlElE", but is encoded
    // either as "_ZTSFvPFvu3i32ES_E" (compressed) or "_ZTSFvPFvu3i64ES_E"
    // (compressed), similarly to the hello_from_c declaration above--this can
    // be ignored for the purposes of this example.
    fn indirect_call_from_c(f: unsafe extern "C" fn(c_long), arg: c_long);
}

// This definition would have the type id "_ZTSFvlE", but is encoded either as
// "_ZTSFvu3i32E" or "_ZTSFvu3i64E", similarly to the hello_from_c declaration
// above.
unsafe extern "C" fn hello_from_rust(_: c_long) {
    println!("Hello, world!");
}

// This definition would have the type id "_ZTSFvlE", but is encoded either as
// "_ZTSFvu3i32E" or "_ZTSFvu3i64E", similarly to the hello_from_c declaration
// above.
unsafe extern "C" fn hello_from_rust_again(_: c_long) {
    println!("Hello from Rust again!");
}

// This definition would also have the type id "_ZTSFvPFvlElE", but is encoded
// either as "_ZTSFvPFvu3i32ES_E" (compressed) or "_ZTSFvPFvu3i64ES_E"
// (compressed), similarly to the hello_from_c declaration above--this can be
// ignored for the purposes of this example.
fn indirect_call(f: unsafe extern "C" fn(c_long), arg: c_long) {
    // This indirect call site tests whether the destination pointer is a member
    // of the group derived from the same type id of the f declaration, which
    // would have the type id "_ZTSFvlE", but is encoded either as
    // "_ZTSFvu3i32E" or "_ZTSFvu3i64E", similarly to the hello_from_c
    // declaration above.
    //
    // Notice that since the test is at the call site and generated by the Rust
    // compiler, the type id used in the test is encoded by the Rust compiler.
    unsafe { f(arg) }
}

// This definition has the type id "_ZTSFvvE"--this can be ignored for the
// purposes of this example.
fn main() {
    // This demonstrates an indirect call within Rust-only code using the same
    // encoding for hello_from_rust and the test at the indirect call site at
    // indirect_call (i.e., "_ZTSFvu3i32E" or "_ZTSFvu3i64E").
    indirect_call(hello_from_rust, 5);

    // This demonstrates an indirect call across the FFI boundary with the Rust
    // compiler and Clang using different encodings for hello_from_c and the
    // test at the indirect call site at indirect_call (i.e., "_ZTSFvu3i32E" or
    // "_ZTSFvu3i64E" vs "_ZTSFvlE").
    //
    // When using rustc LTO (i.e., -Clto), this works because the type id used
    // is from the Rust-declared hello_from_c, which is encoded by the Rust
    // compiler (i.e., "_ZTSFvu3i32E" or "_ZTSFvu3i64E").
    //
    // When using (proper) LTO (i.e., -Clinker-plugin-lto), this does not work
    // because the type id used is from the C-defined hello_from_c, which is
    // encoded by Clang (i.e., "_ZTSFvlE").
    indirect_call(hello_from_c, 5);

    // This demonstrates an indirect call to a function passed as a callback
    // across the FFI boundary with the Rust compiler and Clang using different
    // encodings for the passed-callback declaration and the test at the
    // indirect call site at indirect_call_from_c (i.e., "_ZTSFvu3i32E" or
    // "_ZTSFvu3i64E" vs "_ZTSFvlE").
    //
    // When Rust functions are passed as callbacks across the FFI boundary to be
    // called back from C code, the tests are also at the call site but
    // generated by Clang instead, so the type ids used in the tests are encoded
    // by Clang, which will not match the type ids of declarations encoded by
    // the Rust compiler (e.g., hello_from_rust_again). (The same happens the
    // other way around for C functions passed as callbacks across the FFI
    // boundary to be called back from Rust code.)
    unsafe {
        indirect_call_from_c(hello_from_rust_again, 5);
    }
}

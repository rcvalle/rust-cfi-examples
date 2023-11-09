rust-cfi-examples
=================

The LLVM CFI support in the Rust compiler provides forward-edge control flow
protection for both Rust-compiled code only and for C or C++ and Rust -compiled
code mixed-language binaries, also known as “mixed binaries” (i.e., for when C
or C++ and Rust -compiled code share the same virtual address space), by
aggregating function pointers in groups identified by their return and
parameter types.

LLVM CFI can be enabled with `-Zsanitizer=cfi` and requires LTO (i.e.,
`-Clinker-plugin-lto` or `-Clto`). Cross-language LLVM CFI can be enabled with
`-Zsanitizer=cfi`, and requires the `-Zsanitizer-cfi-normalize-integers` option
to be used with Clang `-fsanitize-cfi-icall-experimental-normalize-integers`
option for cross-language LLVM CFI support, and proper (i.e., non-rustc) LTO
(i.e., `-Clinker-plugin-lto`).

It is recommended to rebuild the standard library with CFI enabled by using the
Cargo build-std feature (i.e., `-Zbuild-std`) when enabling CFI.


Example 1: Redirecting control flow using an indirect branch/call to an invalid
destination
-------------------------------------------------------------------------------

```rust
#![feature(naked_functions)]

use std::arch::asm;
use std::mem;

fn add_one(x: i32) -> i32 {
    x + 1
}

#[naked]
pub extern "C" fn add_two(x: i32) {
    // x + 2 preceded by a landing pad/nop block
    unsafe {
        asm!(
            "
             nop
             nop
             nop
             nop
             nop
             nop
             nop
             nop
             nop
             lea eax, [rdi+2]
             ret
        ",
            options(noreturn)
        );
    }
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    println!("With CFI enabled, you should not see the next answer");
    let f: fn(i32) -> i32 = unsafe {
        // Offset 0 is a valid branch/call destination (i.e., the function entry
        // point), but offsets 1-8 within the landing pad/nop block are invalid
        // branch/call destinations (i.e., within the body of the function).
        mem::transmute::<*const u8, fn(i32) -> i32>((add_two as *const u8).offset(5))
    };
    let next_answer = do_twice(f, 5);

    println!("The next answer is: {}", next_answer);
}
```
Fig. 1. Redirecting control flow using an indirect branch/call to an invalid
destination (i.e., within the body of the function).

```text
$ cargo run --release
   Compiling rust-cfi-1 v0.1.0 (/home/rcvalle/rust-cfi-1)
    Finished release [optimized] target(s) in 0.43s
     Running `target/release/rust-cfi-1`
The answer is: 12
With CFI enabled, you should not see the next answer
The next answer is: 14
$
```
Fig. 2. Build and execution of Fig. 1 with LLVM CFI disabled.

```text
$ RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld -Zsanitizer=cfi" cargo run -Zbuild-std -Zbuild-std-features --release --target x86_64-unknown-linux-gnu
   ...
   Compiling rust-cfi-1 v0.1.0 (/home/rcvalle/rust-cfi-1)
    Finished release [optimized] target(s) in 1m 08s
     Running `target/x86_64-unknown-linux-gnu/release/rust-cfi-1`
The answer is: 12
With CFI enabled, you should not see the next answer
Illegal instruction
$
```
Fig. 3. Build and execution of Fig. 1 with LLVM CFI enabled.

When LLVM CFI is enabled, if there are any attempts to redirect control flow
using an indirect branch/call to an invalid destination, the execution is
terminated (see Fig. 10).


Example 2: Redirecting control flow using an indirect branch/call to a function
with a different number of parameters
-------------------------------------------------------------------------------

```rust
use std::mem;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_two(x: i32, _y: i32) -> i32 {
    x + 2
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    println!("With CFI enabled, you should not see the next answer");
    let f: fn(i32) -> i32 =
        unsafe { mem::transmute::<*const u8, fn(i32) -> i32>(add_two as *const u8) };
    let next_answer = do_twice(f, 5);

    println!("The next answer is: {}", next_answer);
}
```
Fig. 4. Redirecting control flow using an indirect branch/call to a function
with a different number of parameters than arguments intended/passed in the
call/branch site.

```text
$ cargo run --release
   Compiling rust-cfi-2 v0.1.0 (/home/rcvalle/rust-cfi-2)
    Finished release [optimized] target(s) in 0.43s
     Running `target/release/rust-cfi-2`
The answer is: 12
With CFI enabled, you should not see the next answer
The next answer is: 14
$
```
Fig. 5. Build and execution of Fig. 4 with LLVM CFI disabled.

```text
$ RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld -Zsanitizer=cfi" cargo run -Zbuild-std -Zbuild-std-features --release --target x86_64-unknown-linux-gnu
   ...
   Compiling rust-cfi-2 v0.1.0 (/home/rcvalle/rust-cfi-2)
    Finished release [optimized] target(s) in 1m 08s
     Running `target/x86_64-unknown-linux-gnu/release/rust-cfi-2`
The answer is: 12
With CFI enabled, you should not see the next answer
Illegal instruction
$
```
Fig. 6. Build and execution of Fig. 4 with LLVM CFI enabled.

When LLVM CFI is enabled, if there are any attempts to redirect control flow
using an indirect branch/call to a function with a different number of
parameters than arguments intended/passed in the call/branch site, the
execution is also terminated (see Fig. 13).


Example 3: Redirecting control flow using an indirect branch/call to a function
with different return and parameter types
-------------------------------------------------------------------------------

```rust
use std::mem;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_two(x: i64) -> i64 {
    x + 2
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    println!("With CFI enabled, you should not see the next answer");
    let f: fn(i32) -> i32 =
        unsafe { mem::transmute::<*const u8, fn(i32) -> i32>(add_two as *const u8) };
    let next_answer = do_twice(f, 5);

    println!("The next answer is: {}", next_answer);
}
```
Fig. 7. Redirecting control flow using an indirect branch/call to a function
with different return and parameter types than the return type expected and
arguments intended/passed at the call/branch site.

```text
$ cargo run --release
   Compiling rust-cfi-3 v0.1.0 (/home/rcvalle/rust-cfi-3)
    Finished release [optimized] target(s) in 0.44s
     Running `target/release/rust-cfi-3`
The answer is: 12
With CFI enabled, you should not see the next answer
The next answer is: 14
$
```
Fig. 8. Build and execution of Fig. 7 with LLVM CFI disabled.

```text
$ RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld -Zsanitizer=cfi" cargo run -Zbuild-std -Zbuild-std-features --release --target x86_64-unknown-linux-gnu
   ...
   Compiling rust-cfi-3 v0.1.0 (/home/rcvalle/rust-cfi-3)
    Finished release [optimized] target(s) in 1m 07s
     Running `target/x86_64-unknown-linux-gnu/release/rust-cfi-3`
The answer is: 12
With CFI enabled, you should not see the next answer
Illegal instruction
$
```
Fig. 9. Build and execution of Fig. 7 with LLVM CFI enabled.

When LLVM CFI is enabled, if there are any attempts to redirect control flow
using an indirect branch/call to a function with different return and parameter
types than the return type expected and arguments intended/passed at the
call/branch site, the execution is also terminated (see Fig. 16).


Example 4: Redirecting control flow using an indirect branch/call to a function
with different return and parameter types across the FFI boundary
-------------------------------------------------------------------------------

```c
int
do_twice(int (*fn)(int), int arg)
{
    return fn(arg) + fn(arg);
}
```
Fig. 10. Example C library.

```rust
use std::mem;

#[link(name = "foo")]
extern "C" {
    fn do_twice(f: unsafe extern "C" fn(i32) -> i32, arg: i32) -> i32;
}

unsafe extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

unsafe extern "C" fn add_two(x: i64) -> i64 {
    x + 2
}

fn main() {
    let answer = unsafe { do_twice(add_one, 5) };

    println!("The answer is: {}", answer);

    println!("With CFI enabled, you should not see the next answer");
    let f: unsafe extern "C" fn(i32) -> i32 = unsafe {
        mem::transmute::<*const u8, unsafe extern "C" fn(i32) -> i32>(add_two as *const u8)
    };
    let next_answer = unsafe { do_twice(f, 5) };

    println!("The next answer is: {}", next_answer);
}
```
Fig. 11. Redirecting control flow using an indirect branch/call to a function
with different return and parameter types than the return type expected and
arguments intended/passed in the call/branch site, across the FFI boundary.

```text
$ make
mkdir -p target/release
clang -I. -Isrc -Wall -c src/foo.c -o target/release/libfoo.o
llvm-ar rcs target/release/libfoo.a target/release/libfoo.o
RUSTFLAGS="-L./target/release -Clinker=clang -Clink-arg=-fuse-ld=lld" cargo build --release
   Compiling rust-cfi-4 v0.1.0 (/home/rcvalle/rust-cfi-4)
    Finished release [optimized] target(s) in 0.49s
$ ./target/release/rust-cfi-4
The answer is: 12
With CFI enabled, you should not see the next answer
The next answer is: 14
$
```
Fig. 12. Build and execution of Figs. 10–11 with LLVM CFI disabled.

```text
$ make
mkdir -p target/release
clang -I. -Isrc -Wall -flto -fsanitize=cfi -fsanitize-cfi-icall-experimental-normalize-integers -fvisibility=hidden -c -emit-llvm src/foo.c -o target/release/libfoo.bc
llvm-ar rcs target/release/libfoo.a target/release/libfoo.bc
RUSTFLAGS="-L./target/release -Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld -Zsanitizer=cfi -Zsanitizer-cfi-normalize-integers" cargo build -Zbuild-std -Zbuild-std-features --release --target x86_64-unknown-linux-gnu
   ...
   Compiling rust-cfi-4 v0.1.0 (/home/rcvalle/rust-cfi-4)
    Finished release [optimized] target(s) in 1m 06s
$ ./target/x86_64-unknown-linux-gnu/release/rust-cfi-4
The answer is: 12
With CFI enabled, you should not see the next answer
Illegal instruction
$
```
Fig. 13. Build and execution of FIgs. 10–11 with LLVM CFI enabled.

When LLVM CFI is enabled, if there are any attempts to redirect control flow
using an indirect branch/call to a function with different return and parameter
types than the return type expected and arguments intended/passed in the
call/branch site, even across the FFI boundary and for `extern "C"` function
types indirectly called (i.e., callbacks/function pointers) across the FFI
boundary, the execution is also terminated (see Fig. 20).

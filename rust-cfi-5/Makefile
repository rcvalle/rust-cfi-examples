SHELL = /bin/sh

.SUFFIXES:
.SUFFIXES: .c .o

AR = llvm-ar
CARGO = cargo
CARGOFLAGS = -Zbuild-std -Zbuild-std-features --release --target x86_64-unknown-linux-gnu
CC = clang
CFLAGS = -Wall -flto -fsanitize=cfi -fvisibility=hidden
RUSTFLAGS = -Clinker-plugin-lto -Clinker=$(CC) -Clink-arg=-fuse-ld=lld -Zsanitizer=cfi

srcdir = src

all: target/x86_64-unknown-linux-gnu/release/rust-cfi-5

target/release/libfoo.bc: $(srcdir)/foo.c
	mkdir -p target/release
	$(CC) -I. -I$(srcdir) $(CFLAGS) -c -emit-llvm $< -o $@

target/release/libfoo.a: target/release/libfoo.bc
	$(AR) rcs $@ $<

target/x86_64-unknown-linux-gnu/release/rust-cfi-5: target/release/libfoo.a $(srcdir)/main.rs Cargo.toml
	RUSTFLAGS="-L./target/release $(RUSTFLAGS)" $(CARGO) build $(CARGOFLAGS)

clean:
	rm -r target/

SHELL = /bin/sh

.SUFFIXES:
.SUFFIXES: .c .o

CARGO = cargo
CARGOFLAGS = -Zbuild-std -Zbuild-std-features --release --target x86_64-unknown-linux-gnu
CC = clang
CFLAGS = -Wall -flto -fsanitize=cfi -fsanitize-cfi-icall-experimental-normalize-integers -fvisibility=hidden
RUSTFLAGS = -Clinker-plugin-lto -Clinker=$(CC) -Clink-arg=-fuse-ld=lld -Zsanitizer=cfi -Zsanitizer-cfi-normalize-integers

srcdir = src

all: target/x86_64-unknown-linux-gnu/release/main

target/x86_64-unknown-linux-gnu/release/libfoo.a: $(srcdir)/lib.rs Cargo.toml
	RUSTFLAGS="$(RUSTFLAGS)" $(CARGO) build $(CARGOFLAGS)

target/x86_64-unknown-linux-gnu/release/main: $(srcdir)/main.c target/x86_64-unknown-linux-gnu/release/libfoo.a
	$(CC) -I. -I$(srcdir) -fuse-ld=lld $(CFLAGS) $^ -o $@ -ldl -lpthread

clean:
	rm -r target/

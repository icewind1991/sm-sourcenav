DIR:=$(strip $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST)))))
.PHONY: target/i686-unknown-linux-gnu/release/libsourcenav.so

all: dist/sourcenav.ext.so

dist/sourcenav.ext.so: target/i686-unknown-linux-gnu/release/libsourcenav.so
	cp target/i686-unknown-linux-gnu/release/libsourcenav.so dist/sourcenav.ext.so
	strip dist/sourcenav.ext.so

target/i686-unknown-linux-gnu/release/libsourcenav.so:
	docker run --rm -v /tmp/sourcenav_cargo_reg:/root/.cargo/registry -v /tmp/sourcenav_cargo_git:/root/.cargo/git \
		-v "$(DIR)":/app icewind1991/old-libc-rust-builder build --release --target=i686-unknown-linux-gnu
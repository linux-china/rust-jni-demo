#!/bin/bash

# install cross
cargo install cross

# x86_64-unknown-linux-gnu
echo "compiling for x86_64-unknown-linux-gnu"
rustup target add x86_64-unknown-linux-gnu
cross build --release --target x86_64-unknown-linux-gnu
ls -la target/x86_64-unknown-linux-gnu/release
cp -f target/x86_64-unknown-linux-gnu/release/libmylib.so ../src/main/resources/libmylib-linux64.so

## x86_64-unknown-linux-musl
#echo "compiling for x86_64-unknown-linux-musl"
#rustup target add x86_64-unknown-linux-musl
#cross build --release --target x86_64-unknown-linux-musl
#ls -la target/x86_64-unknown-linux-musl/release
#cp target/x86_64-unknown-linux-musl/release/libmylib.so java/src/main/resources/libmylib-linux64-musl.so

# x86_64-apple-darwin
echo "compiling for x86_64-apple-darwin"
rustup target add x86_64-apple-darwin
cross build --release --target x86_64-apple-darwin
ls -la target/x86_64-apple-darwin/release
cp target/x86_64-apple-darwin/release/libmylib.dylib ../src/main/resources/libmylib-osx-x86_64.dylib

# x86_64-pc-windows-gnu
echo "compiling for x86_64-pc-windows-gnu"
rustup target add x86_64-pc-windows-gnu
cross build --release --target x86_64-pc-windows-gnu
ls -la target/x86_64-pc-windows-gnu/release
cp -f target/x86_64-pc-windows-gnu/release/mylib.dll ../src/main/resources/libmylib-win64.dll


# aarch64-unknown-linux-gnu
echo "compiling for aarch64-unknown-linux-gnu"
rustup target add aarch64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
ls -la target/x86_64-unknown-linux-gnu/release
cp -f target/x86_64-unknown-linux-gnu/release/libmylib.so ../src/main/resources/libmylib-linux-aarch64.so

## aarch64-apple-darwin
echo "compiling for aarch64-apple-darwin"
rustup target add aarch64-apple-darwin
cross build --release --target aarch64-apple-darwin
ls -la target/x86_64-apple-darwin/release
cp -f target/x86_64-apple-darwin/release/libmylib.dylib ../src/main/resources/libmylib-osx-arm64.dylib

# aarch64-unknown-linux-musl
#echo "compiling for aarch64-unknown-linux-musl"
#rustup target add aarch64-unknown-linux-musl
#cross build --release --target aarch64-unknown-linux-musl
#ls -la target/aarch64-unknown-linux-musl/release
#cp -f target/aarch64-unknown-linux-musl/release/libmylib.so ../src/main/resources/libmylib-linux-aarch64-musl.so
